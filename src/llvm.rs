use crate::analyzer::Analyzer;
use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::environment::VarStore;
use crate::operators::{BinaryOp, LogicalOp, LoopControl, UnaryOp};
use crate::parser::Parser;
use crate::types::Type;
use crate::WabbitType;
use std::collections::{HashMap, HashSet};

// NOTE
// in this module, unlike all others, I prefer panicing to raising an Error
// rationale:
//   a) all errors will be caused by the typechecker
//   b) it makes things look cleaner, and this is a "toy" implementation!!
//
// This may mean in some cases that a failure in the typechecker causes a silent failure

#[derive(PartialEq)]
enum Signal {
    Return,
    Unit,
    Continue,
    Break,
}

const RUNTIME: &str = r#"
@.str = private unnamed_addr constant [9 x i8] c"Out: %i\0A\00", align 1
@.str.1 = private unnamed_addr constant [10 x i8] c"Out: %lf\0A\00", align 1
@.str.2 = private unnamed_addr constant [11 x i8] c"Out: true\0A\00", align 1
@.str.3 = private unnamed_addr constant [12 x i8] c"Out: false\0A\00", align 1
@.str.4 = private unnamed_addr constant [3 x i8] c"%c\00", align 1
@stdout = external local_unnamed_addr global ptr, align 8

; Function Attrs: nounwind sspstrong uwtable
define void @_print_int(i32 noundef %0) local_unnamed_addr #0 {
  %2 = tail call i32 (i32, ptr, ...) @__printf_chk(i32 noundef 1, ptr noundef nonnull @.str, i32 noundef %0) #3
  ret void
}

declare i32 @__printf_chk(i32 noundef, ptr noundef, ...) local_unnamed_addr #1

; Function Attrs: nounwind sspstrong uwtable
define void @_print_float(double noundef %0) local_unnamed_addr #0 {
  %2 = tail call i32 (i32, ptr, ...) @__printf_chk(i32 noundef 1, ptr noundef nonnull @.str.1, double noundef %0) #3
  ret void
}

; Function Attrs: nounwind sspstrong uwtable
define void @_print_bool(i32 noundef %0) local_unnamed_addr #0 {
  %2 = icmp eq i32 %0, 0
  %3 = select i1 %2, ptr @.str.3, ptr @.str.2
  %4 = tail call i32 (i32, ptr, ...) @__printf_chk(i32 noundef 1, ptr noundef nonnull %3) #3
  ret void
}

; Function Attrs: nounwind sspstrong uwtable
define void @_print_char(i8 noundef signext %0) local_unnamed_addr #0 {
  %2 = sext i8 %0 to i32
  %3 = tail call i32 (i32, ptr, ...) @__printf_chk(i32 noundef 1, ptr noundef nonnull @.str.4, i32 noundef %2) #3
  %4 = load ptr, ptr @stdout, align 8, !tbaa !4
  %5 = tail call i32 @fflush(ptr noundef %4)
  ret void
}

; Function Attrs: nofree nounwind
declare noundef i32 @fflush(ptr nocapture noundef) local_unnamed_addr #2

attributes #0 = { nounwind sspstrong uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 2}
!3 = !{!"clang version 15.0.7"}
!4 = !{!5, !5, i64 0}
!5 = !{!"any pointer", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
"#;

pub struct CodegenLLVM<'a> {
    analyze: Analyzer<'a, Type>,
    counter: usize,
    continue_labels: Vec<String>,
    break_labels: Vec<String>,
    var_names: Environment<'a, String>,
    globals: Vec<String>,
    main: Vec<String>,
    func_llvm: Vec<String>,
    global_vars: HashSet<String>,
}

impl<'a> From<&'a Parser<'a>> for CodegenLLVM<'a> {
    fn from(parser: &'a Parser) -> Self {
        Self {
            analyze: Analyzer::from(parser),
            counter: 0,
            continue_labels: Vec::new(),
            break_labels: Vec::new(),
            var_names: Environment::new(),
            globals: Vec::new(),
            main: Vec::new(),
            func_llvm: Vec::new(),
            global_vars: HashSet::new(),
        }
    }
}

// some implementations to convert values/types to LLVM

impl WabbitType {
    pub fn llvm_value(&self) -> String {
        match self {
            WabbitType::Int(val) => val.to_string(),
            WabbitType::Bool(val) => val.to_string(),
            WabbitType::Char(val) => (*val as i8).to_string(),
            // there's likely some edge cases, but this is fine by me!
            WabbitType::Float(val) => {
                let bytes = val.to_be_bytes();
                let hex = format!(
                    "0x{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
                );
                hex
            }
            _ => panic!("typecheck failure"),
        }
    }
}

impl Type {
    pub fn global_init(&self) -> String {
        match self {
            Type::Int | Type::Bool | Type::Char => "0".to_string(),
            Type::Float => "0x0".to_string(),
        }
    }

    pub fn llvm_type(&self) -> String {
        match self {
            Type::Int => "i32".to_string(),
            Type::Float => "double".to_string(),
            Type::Bool => "i1".to_string(),
            Type::Char => "i8".to_string(),
        }
    }
}

impl<'a> CodegenLLVM<'a> {
    pub fn llvm_codegen(&mut self) -> String {
        for stmt in self.analyze.statements {
            self.llvm_stmt(stmt);
        }
        let combine = vec![
            "target triple = \"x86_64-unknown-linux-gnu\"".to_string(),
            RUNTIME.to_string(),
            self.func_llvm.join("\n"),
            self.globals.join("\n"),
            "define void @main() \n{\nentry:".to_string(),
            self.main.join("\n"),
            "\tret void\n}".to_string(),
        ];
        combine.join("\n")
    }

    // misc utilities for generating unique names
    // for simpicity, they all share the counter
    fn label_name(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("{}_{}", prefix, self.counter)
    }

    fn tmp_name(&mut self) -> String {
        self.counter += 1;
        format!("%.{}", self.counter)
    }

    fn tmp_no_inc(&self) -> String {
        format!("%.{}", self.counter)
    }

    // determines if we are currently build a function call
    // and gives a (mutable) reference to the corresponding Vec
    fn loc(&mut self) -> &mut Vec<String> {
        if self.analyze.call_depth > 0 {
            &mut self.func_llvm
        } else {
            &mut self.main
        }
    }

    // utilities to use both the type and name environment at the same time
    fn enter_child(&mut self) {
        self.analyze.env.enter_child();
        self.var_names.enter_child();
    }

    fn exit_child_unwrap(&mut self) {
        self.analyze.env.exit_child_unwrap();
        self.var_names.exit_child_unwrap();
    }

    // utility for generating binary instructions
    fn binary_ops(&self, t: &Type, op: &BinaryOp) -> String {
        let s = match (t, op) {
            (Type::Int, BinaryOp::Plus) => "add",
            (Type::Int, BinaryOp::Minus) => "sub",
            (Type::Int, BinaryOp::Times) => "mul",
            (Type::Int, BinaryOp::Divide) => "sdiv",
            (Type::Float, BinaryOp::Plus) => "fadd",
            (Type::Float, BinaryOp::Minus) => "fsub",
            (Type::Float, BinaryOp::Times) => "fmul",
            (Type::Float, BinaryOp::Divide) => "fdiv",
            (Type::Bool | Type::Int | Type::Char, BinaryOp::EqualEqual) => "icmp eq",
            (Type::Bool | Type::Int | Type::Char, BinaryOp::NotEqual) => "icmp ne",
            (Type::Float, BinaryOp::EqualEqual) => "fcmp oeq",
            (Type::Float, BinaryOp::NotEqual) => "fcmp one",
            (Type::Int | Type::Char, BinaryOp::Less) => "icmp slt",
            (Type::Int | Type::Char, BinaryOp::LessEqual) => "icmp sle",
            (Type::Int | Type::Char, BinaryOp::Greater) => "icmp sgt",
            (Type::Int | Type::Char, BinaryOp::GreaterEqual) => "icmp sge",
            (Type::Float, BinaryOp::Less) => "fcmp olt",
            (Type::Float, BinaryOp::LessEqual) => "fcmp ole",
            (Type::Float, BinaryOp::Greater) => "fcmp ogt",
            (Type::Float, BinaryOp::GreaterEqual) => "fcmp oge",
            _ => panic!("typecheck failure"),
        };
        s.to_string()
    }

    fn llvm_stmt(&mut self, stmt: &'a Stmt) -> Signal {
        match stmt {
            Stmt::LoopControl { control, .. } => match control {
                LoopControl::Break => Signal::Break,
                LoopControl::Continue => Signal::Continue,
            },
            Stmt::While {
                condition, body, ..
            } => {
                // we make a label for the test, which we will return to until the condition is met
                let test_label = self.label_name("while_cond");
                let after_label = self.label_name("after_while");
                let body_label = self.label_name("while_body");

                self.break_labels.push(after_label.clone());
                self.continue_labels.push(test_label.clone());

                // first the test label, check the condition
                // we either jump to the body, or to the after
                self.loc().push(format!("\tbr label %{}", test_label));
                self.loc().push(format!("{}:", test_label));
                let cond_compile = self.llvm_expr(condition);
                self.loc().push(format!(
                    "\tbr i1 {}, label %{}, label %{}",
                    cond_compile, body_label, after_label
                ));

                self.analyze.loop_depth += 1;
                self.enter_child();
                // now create the body, at the end of which we go back to the test
                self.loc().push(format!("{}:", body_label));
                self.llvm_stmt(body);
                self.loc().push(format!("\tbr label %{}", test_label));
                self.analyze.loop_depth -= 1;
                self.exit_child_unwrap();

                // and lastly, the rest of the program
                self.loc().push(format!("{}:", after_label));
                Signal::Unit
            }
            Stmt::If {
                condition,
                then_block,
                maybe_else_block,
                ..
            } => {
                let cond_compile = self.llvm_expr(condition);

                // label for then_block
                let after_label = self.label_name("after_if");
                let then_label = self.label_name("then");

                // I assume that return signals match up
                let mut has_return = false;

                if let Some(else_block) = maybe_else_block {
                    let else_label = self.label_name("else");

                    // the comparison, jumping to either then/else
                    self.loc().push(format!(
                        "\tbr i1 {}, label %{}, label %{}",
                        cond_compile, then_label, else_label
                    ));

                    // both blocks, after which they both go to the after label
                    self.loc().push(format!("{}:", then_label));
                    self.enter_child();
                    let then_signal = self.llvm_stmt(then_block);

                    if then_signal == Signal::Return {
                        has_return = true;
                    }

                    self.exit_child_unwrap();

                    if !has_return {
                        self.loc().push(format!("\tbr label %{}", after_label));
                    }

                    self.loc().push(format!("{}:", else_label));
                    self.enter_child();
                    self.llvm_stmt(else_block);

                    if !has_return {
                        self.loc().push(format!("\tbr label %{}", after_label));
                    }

                    self.exit_child_unwrap();
                } else {
                    // the comparison, jumping to after if false since no else
                    self.loc().push(format!(
                        "\tbr i1 {}, label %{}, label %{}",
                        cond_compile, then_label, after_label
                    ));

                    // just the then block
                    self.loc().push(format!("{}:", then_label));
                    self.enter_child();
                    self.llvm_stmt(then_block);
                    self.exit_child_unwrap();
                    self.loc().push(format!("\tbr label %{}", after_label));
                }
                if !has_return {
                    self.loc().push(format!("{}:", after_label));
                }
                Signal::Unit
            }
            Stmt::Expr(e) => {
                self.llvm_expr(e);
                Signal::Unit
            }
            Stmt::Assign { name, value, .. } => {
                let name_lookup = self.var_names.get(name).unwrap().clone_store();
                let dtype = self.analyze.expr_type(value).unwrap();
                let llvm_type = dtype.llvm_type();
                let compiled_value = self.llvm_expr(value);
                self.analyze.env.assign(name, dtype);

                if self.global_vars.contains(&name_lookup) {
                    self.loc().push(format!(
                        "\tstore {} {}, {}* @{}",
                        llvm_type, compiled_value, llvm_type, name
                    ));
                } else {
                    // in case it was previously only declared
                    self.analyze.env.assign(name, dtype);
                    let name = self.var_names.get(name).unwrap().clone_store();
                    self.loc().push(format!(
                        "\tstore {} {}, {}* {}",
                        dtype.llvm_type(),
                        compiled_value,
                        dtype.llvm_type(),
                        name
                    ));
                }
                Signal::Unit
            }
            Stmt::FuncDef {
                def_name,
                def_params,
                return_type,
                body,
                ..
            } => {
                let return_type = return_type.llvm_type();
                let mut param_names: HashMap<&String, VarStore<String>> = HashMap::new();
                let mut param_types: HashMap<&String, VarStore<Type>> = HashMap::new();
                let mut args = Vec::new();
                let mut queue = Vec::new();

                for (name, dtype) in def_params {
                    let tmp_name = self.tmp_name();
                    let llvm_type = dtype.llvm_type();

                    args.push(format!("{} {}", llvm_type, tmp_name));
                    queue.push(format!("\t%{} = alloca {}", name, llvm_type));
                    queue.push(format!(
                        "\tstore {} {}, {}* %{}",
                        llvm_type, tmp_name, llvm_type, name
                    ));

                    param_names.insert(name, VarStore::Init(format!("%{}", name)));
                    param_types.insert(name, VarStore::Init(*dtype));
                }

                self.func_llvm.push(format!(
                    "\ndefine {} @{} ({})\n{{",
                    return_type,
                    def_name,
                    args.join(", ")
                ));

                for item in queue {
                    self.func_llvm.push(item);
                }

                self.analyze.functions.insert(def_name, stmt);
                self.analyze.env.enter_child_fn(param_types);
                self.var_names.enter_child_fn(param_names);
                self.analyze.call_depth += 1;

                self.llvm_stmt(body);

                self.analyze.env.exit_child_unwrap();
                self.var_names.exit_child_unwrap();
                self.analyze.call_depth -= 1;
                self.func_llvm.push("}\n".to_string());

                Signal::Unit
            }
            Stmt::Block { statements, .. } => {
                let mut signal = Signal::Unit;

                for stmt in statements {
                    signal = self.llvm_stmt(stmt);
                    match signal {
                        Signal::Unit => (),
                        Signal::Return => {
                            break;
                        }
                        Signal::Break | Signal::Continue => {
                            let label = if signal == Signal::Break {
                                self.break_labels.pop().unwrap()
                            } else {
                                self.continue_labels.pop().unwrap()
                            };
                            let jmp = format!("\tbr label %{}", label);
                            self.loc().push(jmp);
                            break;
                        }
                    }
                }
                // if we got to the end of a loop w/o break/continue, pop off those labels
                if self.analyze.loop_depth > self.break_labels.iter().len() {
                    self.break_labels.pop();
                };
                if self.analyze.loop_depth > self.continue_labels.iter().len() {
                    self.continue_labels.pop();
                };
                signal
            }
            Stmt::Return { value, .. } => {
                let value_compile = self.llvm_expr(value);
                let llvm_type = self.analyze.expr_type(value).unwrap().llvm_type();
                self.loc()
                    .push(format!("\tret {} {}", llvm_type, value_compile));
                Signal::Return
            }
            Stmt::VarDef {
                name,
                maybe_type,
                maybe_value,
                ..
            } => {
                // first, determine the type
                let dtype = if let Some(t) = maybe_type {
                    *t
                } else {
                    self.analyze
                        .expr_type(&maybe_value.clone().unwrap())
                        .unwrap()
                };

                let llvm_type = dtype.llvm_type();
                self.analyze.env.define_init(name, dtype);
                let tmp_name = self.tmp_name();

                // the definition varies depending on the scope
                if self.analyze.env.in_global_scope() {
                    let fmt_name = format!("@{}", name);
                    self.globals.push(format!(
                        "{} = global {} {}",
                        fmt_name,
                        llvm_type,
                        dtype.global_init()
                    ));
                    self.global_vars.insert(fmt_name.clone());
                    self.var_names.define_init(name, fmt_name);
                } else {
                    self.loc()
                        .push(format!("\t{} = alloca {}", tmp_name, llvm_type));
                    self.var_names.define_init(name, tmp_name.clone());
                };

                if let Some(value) = maybe_value {
                    let compiled_value = self.llvm_expr(value);
                    self.analyze.env.define_init(name, dtype);

                    if self.analyze.env.in_global_scope() {
                        self.loc().push(format!(
                            "\tstore {} {}, {}* @{}",
                            llvm_type, compiled_value, llvm_type, name
                        ));
                    } else {
                        self.loc().push(format!(
                            "\tstore {} {}, {}* {}",
                            llvm_type, compiled_value, llvm_type, tmp_name,
                        ));
                    }
                }

                Signal::Unit
            }
            Stmt::ConstDef { name, value, .. } => {
                let dtype = self.analyze.expr_type(value).expect("typechecking failure");
                let llvm_type = dtype.llvm_type();
                let compiled_value = self.llvm_expr(value);
                self.globals.push(format!(
                    "@{} = global {} {}",
                    name,
                    llvm_type,
                    dtype.global_init()
                ));
                self.global_vars.insert(name.to_string());
                self.analyze.env.define_init(name, dtype);
                self.var_names.define_init(name, format!("@{}", name));
                self.analyze.constants.insert(name, dtype);
                self.loc().push(format!(
                    "\tstore {} {}, {}* @{}",
                    llvm_type, compiled_value, llvm_type, name
                ));
                Signal::Unit
            }
            Stmt::Print { value, .. } => {
                let dtype = self.analyze.expr_type(value).unwrap();
                let to_print = self.llvm_expr(value);
                let print = match dtype {
                    Type::Int => format!("\tcall void @_print_int(i32 {})", to_print),
                    Type::Float => format!("\tcall void @_print_float(double {})", to_print),
                    Type::Char => format!("\tcall void @_print_char(i8 {})", to_print),
                    Type::Bool => format!("\tcall void @_print_bool(i1 {})", to_print),
                };
                self.loc().push(print);
                Signal::Unit
            }
        }
    }

    fn llvm_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Literal { value, .. } => value.llvm_value(),
            Expr::Call { name, params, .. } => {
                let func = self.analyze.functions.get(name).unwrap();
                let mut param_str = Vec::new();

                if let Stmt::FuncDef {
                    def_params,
                    return_type,
                    ..
                } = func
                {
                    let llvm_return_type = return_type.llvm_type();

                    for (call_e, (_, param_type)) in std::iter::zip(params, def_params) {
                        let pname = self.llvm_expr(call_e);
                        let ptype = param_type.llvm_type();
                        param_str.push(format!("{} {}", ptype, pname));
                    }

                    let tmp_name = self.tmp_name();
                    self.loc().push(format!(
                        "\t{} = call {} @{}({})",
                        tmp_name,
                        llvm_return_type,
                        name,
                        param_str.join(", ")
                    ));
                    tmp_name
                } else {
                    panic!("typecheck failure")
                }
            }
            Expr::VarName { name, .. } => {
                let dtype = self.analyze.expr_type(e).unwrap();
                let name = self.var_names.get(name).unwrap().clone_store();
                let tmp_name = self.tmp_name();

                self.loc().push(format!(
                    "\t{} = load {}, {}* {}",
                    tmp_name,
                    dtype.llvm_type(),
                    dtype.llvm_type(),
                    name
                ));
                tmp_name
            }
            Expr::TypeConversion { dtype, params, .. } => {
                // there should be a single parameter
                let mut tmp_name = self.tmp_name();
                let param_compile = self.llvm_expr(&params[0]);
                let param_type = self.analyze.expr_type(&params[0]).unwrap();

                if dtype == &param_type {
                    tmp_name = param_compile
                } else {
                    let ins = match (dtype, param_type) {
                        (Type::Int, Type::Char) => {
                            format!("\t{} = zext i8 {} to i32", tmp_name, param_compile)
                        }
                        (Type::Int, Type::Bool) => {
                            format!("\t{} = zext i1 {} to i32", tmp_name, param_compile)
                        }
                        (Type::Int, Type::Float) => {
                            format!("\t{} = fptosi double {} to i32", tmp_name, param_compile)
                        }
                        (Type::Float, Type::Int) => {
                            format!("\t{} = sitofp i32 {} to double", tmp_name, param_compile)
                        }
                        (Type::Char, Type::Int) => {
                            format!("\t{} = trunc i32 {} to i8", tmp_name, param_compile)
                        }
                        (Type::Bool, Type::Int) => {
                            format!("\t{} = trunc i32 {} to i1", tmp_name, param_compile)
                        }
                        _ => panic!("typecheck failure"),
                    };
                    self.loc().push(ins);
                }
                tmp_name
            }
            Expr::TypeName { .. } => panic!("typecheck failure"),
            Expr::Logical { lhs, op, rhs, .. } => {
                let lhs_compile = self.llvm_expr(lhs);

                let rhs_label = self.label_name("rhs_logical");
                let after_label = self.label_name("after_logical");

                let ins = match op {
                    LogicalOp::LogicalOr => format!(
                        "\tbr i1 {}, label %{}, label %{}",
                        lhs_compile, after_label, rhs_label
                    ),
                    LogicalOp::LogicalAnd => format!(
                        "\tbr i1 {}, label %{}, label %{}",
                        lhs_compile, rhs_label, after_label
                    ),
                };

                self.loc().push(ins);

                self.loc().push(format!("{}:", rhs_label));
                let rhs_compile = self.llvm_expr(rhs);
                self.loc().push(format!("\tbr label %{}", after_label));

                self.loc().push(format!("{}:", after_label));
                rhs_compile
            }
            Expr::Grouping { e, .. } => {
                self.llvm_expr(e);
                self.tmp_no_inc()
            }
            Expr::Unary { op, operand, .. } => {
                let operand_compile = self.llvm_expr(operand);
                let operand_type = self.analyze.expr_type(e).unwrap();

                let tmp_name = if op == &UnaryOp::Plus {
                    self.tmp_no_inc()
                } else {
                    self.tmp_name()
                };

                // Unary Plus is a noop
                if op != &UnaryOp::Plus {
                    let ins = match (operand_type, op) {
                        (Type::Int, UnaryOp::Minus) => {
                            format!("\t{} = mul i32 {}, -1", tmp_name, operand_compile)
                        }
                        (Type::Float, UnaryOp::Minus) => {
                            format!("\t{} = fneg double {}", tmp_name, operand_compile)
                        }
                        (Type::Bool, UnaryOp::LogicalNot) => {
                            format!("\t{} = icmp eq i1 {}, 0", tmp_name, operand_compile)
                        }
                        _ => panic!("typecheck failure"),
                    };
                    self.loc().push(ins);
                }

                tmp_name
            }
            Expr::Binary { lhs, op, rhs, .. } => {
                let lhs_compile = self.llvm_expr(lhs);
                let rhs_compile = self.llvm_expr(rhs);

                let dtype = self.analyze.expr_type(lhs).unwrap();
                let llvm_type = dtype.llvm_type();

                let tmp_name = self.tmp_name();

                let op = self.binary_ops(&dtype, op);

                self.loc().push(format!(
                    "\t{} = {} {} {}, {}",
                    tmp_name, op, llvm_type, lhs_compile, rhs_compile
                ));
                tmp_name
            }
        }
    }
}
