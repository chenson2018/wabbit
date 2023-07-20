use crate::analyzer::Analyzer;
use crate::ast::{Expr, Stmt};
use crate::environment::VarStore;
use crate::error::{msg, Msg, RangeReporter, Result};
use crate::operators::*;
use crate::types::{compare, equality, numeric_binary, numeric_unary, Type, WabbitType};
use std::collections::HashMap;

/// struct for interpreting Wabbit AST
pub type Interpreter<'a> = Analyzer<'a, WabbitType>;

#[derive(Debug)]
pub enum Signal {
    Break,
    Continue,
    Unit,
    Return(WabbitType),
}

impl<'a> Interpreter<'a> {
    /// interpret an expression and confirm it is a boolean at runtime
    fn typecheck_bool(&mut self, e: &Expr, id: &usize) -> Result<bool> {
        let eval = self.evaluate(e)?;
        match eval {
            WabbitType::Bool(val) => Ok(val),
            _ => {
                msg!(Msg::ExpectType, (self.ranges, id), "bool")
            }
        }
    }

    /// interpret all statements
    pub fn interpret(&mut self) -> Result<()> {
        for statement in self.statements {
            let _ = self.run_stmt(statement)?;
        }
        Ok(())
    }

    /// interpret a single statement
    fn run_stmt(&mut self, stmt: &'a Stmt) -> Result<Signal> {
        match stmt {
            Stmt::FuncDef {
                ref def_name, id, ..
            } => {
                if !self.env.in_global_scope() {
                    msg!(Msg::FuncDefScope, (self.ranges, id))
                } else {
                    self.check_constant(def_name, id)?;
                    self.check_env(def_name, id)?;
                    self.check_function(def_name, id)?;
                    self.functions.insert(def_name, stmt);
                    Ok(Signal::Unit)
                }
            }
            Stmt::Return { value, id } => {
                let value = self.evaluate(value)?;
                if self.call_depth > 0 {
                    Ok(Signal::Return(value))
                } else {
                    msg!(Msg::ReturnScope, (self.ranges, id))
                }
            }
            Stmt::Expr(e) => {
                self.evaluate(e)?;
                Ok(Signal::Unit)
            }
            Stmt::Assign { name, value, id } => {
                self.check_constant(name, id)?;
                self.check_function(name, id)?;

                let e = self.evaluate(value)?;

                match self.env.get(name) {
                    Some(VarStore::Init(value) | VarStore::UnInit(value)) => {
                        if value.dtype() == e.dtype() {
                            self.env.assign(name, e);
                        } else {
                            return msg!(
                                Msg::AssignRetype,
                                (self.ranges, id),
                                name,
                                value.dtype(),
                                e.dtype()
                            );
                        }
                    }
                    None => return msg!(Msg::AssignUndefined, (self.ranges, id)),
                };
                Ok(Signal::Unit)
            }
            Stmt::While {
                condition,
                body,
                id,
            } => {
                while self.typecheck_bool(condition, id)? {
                    self.loop_depth += 1;
                    self.env.enter_child();
                    let signal = self.run_stmt(body)?;
                    match signal {
                        Signal::Unit => (),
                        Signal::Return(_) => {
                            self.env.exit_child(&(self.ranges, id))?;
                            self.loop_depth -= 1;
                            return Ok(signal);
                        }
                        Signal::Break => {
                            self.env.exit_child(&(self.ranges, id))?;
                            self.loop_depth -= 1;
                            break;
                        }
                        Signal::Continue => {
                            continue;
                        }
                    }
                    self.loop_depth -= 1;
                    self.env.exit_child(&(self.ranges, id))?;
                }
                Ok(Signal::Unit)
            }
            Stmt::ConstDef {
                name,
                maybe_type,
                value,
                id,
            } => {
                self.check_constant(name, id)?;
                self.check_env(name, id)?;
                self.check_function(name, id)?;

                if !self.env.in_global_scope() {
                    msg!(Msg::ConstScope, (self.ranges, id))
                } else {
                    let value = self.evaluate(value)?;
                    if maybe_type.is_none() || (*maybe_type).unwrap() == value.dtype() {
                        self.constants.insert(name, value);
                        Ok(Signal::Unit)
                    } else {
                        msg!(Msg::InitType, (self.ranges, id))
                    }
                }
            }
            Stmt::VarDef {
                name,
                maybe_type,
                maybe_value,
                id,
            } => {
                self.check_constant(name, id)?;
                self.check_env(name, id)?;
                self.check_function(name, id)?;

                match (maybe_type, maybe_value) {
                    (Some(typename), Some(value)) => {
                        // if given a type and value, check they are compatible
                        let value = self.evaluate(value)?;

                        if typename == &value.dtype() {
                            self.env.define_init(name, value);
                        } else {
                            return msg!(Msg::InitType, (self.ranges, id));
                        }
                    }
                    (None, Some(value)) => {
                        let value = self.evaluate(value)?;
                        self.env.define_init(name, value);
                    }
                    (Some(typename), None) => {
                        self.env
                            .define_uninit(name, WabbitType::TypeHolder(*typename));
                    }
                    (None, None) => {
                        return msg!(
                            Msg::InternalErr,
                            (self.ranges, id),
                            "Parser allowed variable definition without type or initial value"
                        );
                    }
                };
                Ok(Signal::Unit)
            }
            Stmt::If {
                condition,
                then_block,
                maybe_else_block,
                id,
            } => {
                let condition = self.typecheck_bool(condition, id)?;

                self.env.enter_child();

                let signal = if condition {
                    self.run_stmt(then_block)?
                } else if let Some(else_block) = maybe_else_block {
                    self.run_stmt(else_block)?
                } else {
                    Signal::Unit
                };

                self.env.exit_child(&(self.ranges, id))?;
                Ok(signal)
            }
            Stmt::Block { statements, .. } => {
                for stmt in statements {
                    let signal = self.run_stmt(stmt)?;
                    match signal {
                        Signal::Unit => (),
                        _ => return Ok(signal),
                    }
                }
                Ok(Signal::Unit)
            }
            Stmt::LoopControl { control, id } => {
                if self.loop_depth > 0 {
                    match control {
                        LoopControl::Break => Ok(Signal::Break),
                        LoopControl::Continue => Ok(Signal::Continue),
                    }
                } else {
                    msg!(Msg::LoopReq, (self.ranges, id))
                }
            }
            Stmt::Print { value, .. } => {
                let value = self.evaluate(value)?;
                match value {
                    WabbitType::Char(_) => print!("{value}"),
                    _ => println!("{value}"),
                };
                self.output.push(value);
                Ok(Signal::Unit)
            }
        }
    }

    /// interpret a single expression
    #[allow(unused_parens)]
    fn evaluate(&mut self, e: &Expr) -> Result<WabbitType> {
        match e {
            Expr::TypeConversion { dtype, params, id } => {
                match params.as_slice() {
                    [to_convert] => {
                        let eval = self.evaluate(to_convert)?;
                        match (dtype, &eval) {
                            (Type::Int, WabbitType::Char(c)) => Ok(WabbitType::Int(*c as i32)),
                            (Type::Int, WabbitType::Bool(b)) => Ok(WabbitType::Int(*b as i32)),
                            (Type::Int, WabbitType::Float(f)) => Ok(WabbitType::Int(*f as i32)),
                            (Type::Float, WabbitType::Int(i)) => Ok(WabbitType::Float(*i as f64)),
                            (Type::Char, WabbitType::Int(i)) => {
                                Ok(WabbitType::Char((*i as u8) as char))
                            }
                            (Type::Bool, WabbitType::Int(i)) => Ok(WabbitType::Bool(*i == 1)),
                            // note sure if in the spec, but this seems reasonable...
                            (Type::Bool, WabbitType::Bool(_))
                            | (Type::Int, WabbitType::Int(_))
                            | (Type::Float, WabbitType::Float(_))
                            | (Type::Char, WabbitType::Char(_)) => Ok(eval),
                            _ => msg!(Msg::TypeConvert, (self.ranges, id)),
                        }
                    }
                    _ => msg!(Msg::ConvertAirty, (self.ranges, id)),
                }
            }
            Expr::Call {
                name: call_name,
                params: call_params,
                id,
            } => {
                let func_lookup = self.functions.get(call_name);

                if let Some(Stmt::FuncDef {
                    def_params,
                    return_type,
                    box body,
                    id,
                    ..
                }) = func_lookup
                {
                    // first check airty
                    let call_airty = call_params.len();
                    let def_airty = def_params.len();

                    if def_airty != call_airty {
                        msg!(
                            Msg::FuncAirty,
                            (self.ranges, id),
                            call_name,
                            def_airty,
                            call_airty
                        )
                    } else {
                        // next check that all parameters have the correct type
                        // if they do, we evaluate them and add to the child environment
                        let mut evaluated_params: HashMap<&String, VarStore<WabbitType>> =
                            HashMap::new();

                        for (call_expr, (def_name, def_type)) in
                            std::iter::zip(call_params, def_params)
                        {
                            let call_expr_eval = self.evaluate(call_expr)?;

                            if def_type != &call_expr_eval.dtype() {
                                return msg!(
                                    Msg::ParamType,
                                    (self.ranges, id),
                                    &def_name,
                                    def_type,
                                    call_expr_eval.dtype()
                                );
                            } else {
                                evaluated_params.insert(def_name, VarStore::Init(call_expr_eval));
                            }
                        }

                        if def_params.len() != evaluated_params.len() {
                            return msg!(Msg::DupArgs, (self.ranges, id));
                        };

                        self.env.enter_child_fn(evaluated_params);
                        self.call_depth += 1;

                        let signal = self.run_stmt(body)?;

                        self.env.exit_child(&(self.ranges, id))?;
                        self.call_depth -= 1;

                        if let Signal::Return(call_return) = signal {
                            if return_type != &call_return.dtype() {
                                msg!(
                                    Msg::ReturnType,
                                    (self.ranges, id),
                                    &call_name,
                                    return_type,
                                    call_return.dtype()
                                )
                            } else {
                                Ok(call_return)
                            }
                        } else {
                            msg!(Msg::NoReturn, (self.ranges, id))
                        }
                    }
                } else {
                    msg!(Msg::FuncUndefined, (self.ranges, id))
                }
            }
            Expr::TypeName { id, .. } => msg!(Msg::TypeEval, (self.ranges, id)),
            Expr::Logical { lhs, op, rhs, id } => {
                let eval_lhs = self.typecheck_bool(lhs, id)?;
                match op {
                    LogicalOp::LogicalOr { .. } => {
                        if eval_lhs {
                            Ok(WabbitType::Bool(true))
                        } else {
                            let eval_rhs = self.typecheck_bool(rhs, id)?;
                            Ok(WabbitType::Bool(eval_lhs || eval_rhs))
                        }
                    }
                    LogicalOp::LogicalAnd { .. } => {
                        if !eval_lhs {
                            Ok(WabbitType::Bool(false))
                        } else {
                            let eval_rhs = self.typecheck_bool(rhs, id)?;
                            Ok(WabbitType::Bool(eval_lhs && eval_rhs))
                        }
                    }
                }
            }
            Expr::VarName { name, id } => {
                if let Some(constant) = self.constants.get(name) {
                    Ok(*constant)
                } else if let Some(VarStore::Init(var)) = self.env.get(name) {
                    Ok(var)
                } else if let Some(VarStore::UnInit(_)) = self.env.get(name) {
                    msg!(Msg::AccessUninit, (self.ranges, id), name)
                } else {
                    msg!(Msg::VarUndefined, (self.ranges, id))
                }
            }
            Expr::Grouping { e, .. } => Ok(self.evaluate(e)?),
            Expr::Unary { op, operand, id } => {
                let eval_operand = self.evaluate(operand)?;
                match op {
                    UnaryOp::LogicalNot => {
                        let raw_operand = self.typecheck_bool(operand, id)?;
                        Ok(WabbitType::Bool(!raw_operand))
                    }
                    UnaryOp::Plus => numeric_unary!(eval_operand, (self.ranges, id), (|a| a)),
                    UnaryOp::Minus => numeric_unary!(eval_operand, (self.ranges, id), (|a| -a)),
                }
            }
            Expr::Binary { lhs, op, rhs, id } => {
                let eval_lhs = self.evaluate(lhs)?;
                let eval_rhs = self.evaluate(rhs)?;

                if !(eval_lhs.dtype() == eval_rhs.dtype()) {
                    msg!(Msg::TypeMatch, (self.ranges, id))
                } else {
                    match op {
                        BinaryOp::Plus => {
                            numeric_binary!(eval_lhs, eval_rhs, (self.ranges, id), +)
                        }
                        BinaryOp::Minus => {
                            numeric_binary!(eval_lhs, eval_rhs, (self.ranges, id), -)
                        }
                        BinaryOp::Times => {
                            numeric_binary!(eval_lhs, eval_rhs, (self.ranges, id), *)
                        }
                        BinaryOp::Divide => {
                            numeric_binary!(eval_lhs, eval_rhs, (self.ranges, id), /)
                        }
                        BinaryOp::Less => compare!(eval_lhs, eval_rhs, (self.ranges, id), <),
                        BinaryOp::LessEqual => {
                            compare!(eval_lhs, eval_rhs, (self.ranges, id), <=)
                        }
                        BinaryOp::Greater => compare!(eval_lhs, eval_rhs, (self.ranges, id), >),
                        BinaryOp::GreaterEqual => {
                            compare!(eval_lhs, eval_rhs, (self.ranges, id), >=)
                        }
                        BinaryOp::EqualEqual => {
                            equality!(eval_lhs, eval_rhs, (self.ranges, id), ==)
                        }
                        BinaryOp::NotEqual => {
                            equality!(eval_lhs, eval_rhs, (self.ranges, id), !=)
                        }
                    }
                }
            }
            Expr::Literal { value, .. } => Ok(*value),
        }
    }
}
