use crate::analyzer::Analyzer;
use crate::ast::{Expr, Stmt};
use crate::environment::VarStore;
use crate::error::{msg, Msg, RangeReporter, Result};
use crate::operators::{BinaryOp, UnaryOp};
use crate::types::Type;
use std::collections::HashMap;

/// struct for typechecking Wabbit AST
pub type Typechecker<'a> = Analyzer<'a, Type>;

#[allow(clippy::new_without_default)]
impl<'a> Typechecker<'a> {
    /// typecheck all statements
    pub fn typecheck(&mut self) -> Result<()> {
        for statement in self.statements {
            self.typecheck_stmt(statement)?;
        }
        Ok(())
    }
}

impl<'a> Analyzer<'a, Type> {
    /// typecheck a single statement
    pub(crate) fn typecheck_stmt(&mut self, stmt: &'a Stmt) -> Result<Option<Type>> {
        match stmt {
            Stmt::Assign { name, value, id } => {
                self.check_constant(name, *id)?;
                self.check_function(name, *id)?;

                let new_type = self.expr_type(value)?;

                match self.env.get(name) {
                    Some(VarStore::Init(old_type) | VarStore::UnInit(old_type)) => {
                        if old_type == new_type {
                            self.env.assign(name, new_type);
                        } else {
                            return msg!(
                                Msg::AssignRetype,
                                (self.ranges, *id),
                                name,
                                old_type,
                                new_type
                            );
                        }
                    }
                    None => return msg!(Msg::AssignUndefined, (self.ranges, *id)),
                };
                Ok(None)
            }
            Stmt::Block { statements, id, .. } => {
                let mut return_types = Vec::new();
                let mut return_exclude_if_while = Vec::new();

                for stmt in statements {
                    if let Some(ret_type) = self.typecheck_stmt(stmt)? {
                        return_types.push(ret_type);

                        match stmt {
                            Stmt::While { .. }
                            | Stmt::If {
                                maybe_else_block: None,
                                ..
                            } => (),
                            Stmt::If {
                                then_block,
                                maybe_else_block: Some(else_block),
                                ..
                            } => {
                                let ret_then = self.typecheck_stmt(then_block).unwrap();
                                let ret_else = self.typecheck_stmt(else_block).unwrap();

                                if matches!((ret_then, ret_else), (Some(_), Some(_))) {
                                    return_exclude_if_while.push(ret_type);
                                };
                            }
                            _ => return_exclude_if_while.push(ret_type),
                        }
                    }
                }

                if return_types.is_empty() {
                    Ok(None)
                } else if return_exclude_if_while.is_empty() && self.loop_depth == 0 {
                    msg!(Msg::AltBranch, (self.ranges, *id))
                } else if return_types.iter().all(|item| item == &return_types[0]) {
                    Ok(Some(return_types[0]))
                } else {
                    msg!(Msg::ReturnDiverge, (self.ranges, *id))
                }
            }
            Stmt::While {
                condition,
                body,
                id,
            } => {
                let condition_type = self.expr_type(condition)?;

                if condition_type != Type::Bool {
                    return msg!(Msg::ExpectType, (self.ranges, *id), "bool");
                };

                self.loop_depth += 1;
                self.env.enter_child();

                let maybe_return = self.typecheck_stmt(body)?;

                self.loop_depth -= 1;
                self.env.exit_child(&(self.ranges, *id))?;

                Ok(maybe_return)
            }
            Stmt::Return { value, id } => {
                let value_type = self.expr_type(value)?;
                if self.call_depth > 0 {
                    Ok(Some(value_type))
                } else {
                    msg!(Msg::ReturnScope, (self.ranges, *id))
                }
            }
            Stmt::If {
                condition,
                then_block,
                maybe_else_block,
                id,
            } => {
                let condition_type = self.expr_type(condition)?;

                if condition_type != Type::Bool {
                    return msg!(Msg::ExpectType, (self.ranges, *id), "bool");
                };

                // while typechecking. we dont know which block we enter
                // here I try each of them, each with their own child environment

                self.env.enter_child();
                let maybe_then_return = self.typecheck_stmt(then_block)?;
                self.env.exit_child(&(self.ranges, *id))?;

                if let Some(else_block) = maybe_else_block {
                    self.env.enter_child();
                    let maybe_else_return = self.typecheck_stmt(else_block)?;
                    self.env.exit_child(&(self.ranges, *id))?;

                    // if both blocks return, check they have the same type
                    if let (Some(then_return), Some(else_return)) =
                        (&maybe_then_return, &maybe_else_return)
                    {
                        if then_return != else_return {
                            return msg!(Msg::ReturnDiverge, (self.ranges, *id));
                        }
                    };
                };

                if let Some(then_return) = maybe_then_return {
                    Ok(Some(then_return))
                } else {
                    Ok(None)
                }
            }
            Stmt::LoopControl { id, .. } => {
                if self.loop_depth > 0 {
                    Ok(None)
                } else {
                    msg!(Msg::LoopReq, (self.ranges, *id))
                }
            }
            Stmt::Print { value, .. } => {
                self.expr_type(value)?;
                Ok(None)
            }
            Stmt::ConstDef {
                name,
                maybe_type,
                value,
                id,
            } => {
                self.check_constant(name, *id)?;
                self.check_env(name, *id)?;
                self.check_function(name, *id)?;

                if !self.env.in_global_scope() {
                    msg!(Msg::ConstScope, (self.ranges, *id))
                } else {
                    let value_type = self.expr_type(value)?;
                    if maybe_type.is_none() || (*maybe_type).unwrap() == value_type {
                        self.constants.insert(name, value_type);
                        Ok(None)
                    } else {
                        msg!(Msg::InitType, (self.ranges, *id))
                    }
                }
            }
            Stmt::FuncDef {
                ref def_name,
                return_type,
                def_params,
                body,
                id,
            } => {
                if !self.env.in_global_scope() {
                    msg!(Msg::FuncDefScope, (self.ranges, *id))
                } else {
                    self.check_constant(def_name, *id)?;
                    self.check_env(def_name, *id)?;
                    self.check_function(def_name, *id)?;

                    let param_types: HashMap<&String, VarStore<Type>> = def_params
                        .iter()
                        .map(|(name, t)| (name, VarStore::Init(*t)))
                        .collect();

                    if def_params.len() != param_types.len() {
                        return msg!(Msg::DupArgs, (self.ranges, *id));
                    };

                    // In the interpreter, these checks happen at the call time
                    // Here, however, they happen before the call, which means recursive functions
                    // will not have access to themselves

                    // to solve this, I add the function definition before typechecking the body
                    // I remove the definion on error, though I don't think it really matters (since I return the first error anyway)
                    self.functions.insert(def_name, stmt);

                    self.env.enter_child_fn(param_types);
                    self.call_depth += 1;

                    let body_type = self.typecheck_stmt(body)?;

                    self.env.exit_child(&(self.ranges, *id))?;
                    self.call_depth -= 1;

                    if let Some(call_return) = body_type {
                        if return_type != &call_return {
                            self.functions.remove(def_name);
                            msg!(
                                Msg::ReturnType,
                                (self.ranges, *id),
                                &def_name,
                                return_type,
                                call_return
                            )
                        } else {
                            Ok(None)
                        }
                    } else {
                        self.functions.remove(def_name);
                        msg!(Msg::NoReturn, (self.ranges, *id))
                    }
                }
            }
            Stmt::VarDef {
                name,
                maybe_type,
                maybe_value,
                id,
            } => {
                self.check_constant(name, *id)?;
                self.check_env(name, *id)?;
                self.check_function(name, *id)?;

                match (maybe_type, maybe_value) {
                    (Some(typename), Some(value)) => {
                        // if given a type and value, check they are compatible
                        let value_type = self.expr_type(value)?;

                        if typename == &value_type {
                            self.env.define_init(name, *typename);
                        } else {
                            return msg!(Msg::InitType, (self.ranges, *id));
                        }
                    }
                    (None, Some(value)) => {
                        let value_type = self.expr_type(value)?;
                        self.env.define_init(name, value_type);
                    }
                    (Some(typename), None) => {
                        self.env.define_uninit(name, *typename);
                    }
                    (None, None) => {
                        return msg!(
                            Msg::InternalErr,
                            (self.ranges, *id),
                            "Parser allowed variable definition without type or initial value"
                        );
                    }
                };
                Ok(None)
            }
            Stmt::Expr(e) => {
                self.expr_type(e)?;
                Ok(None)
            }
        }
    }

    /// typecheck a single expression
    pub(crate) fn expr_type(&mut self, e: &Expr) -> Result<Type> {
        match e {
            Expr::Call {
                name: call_name,
                params: call_params,
                id,
            } => {
                let func_lookup = self.functions.get(call_name);

                if let Some(Stmt::FuncDef {
                    def_params,
                    return_type,
                    id,
                    ..
                }) = func_lookup
                {
                    let call_airty = call_params.len();
                    let def_airty = def_params.len();

                    if def_airty != call_airty {
                        msg!(
                            Msg::FuncAirty,
                            (self.ranges, *id),
                            call_name,
                            def_airty,
                            call_airty
                        )
                    } else {
                        for (call_expr, (def_name, def_type)) in
                            std::iter::zip(call_params, def_params)
                        {
                            let call_expr_type = self.expr_type(call_expr)?;
                            if def_type != &call_expr_type {
                                return msg!(
                                    Msg::ParamType,
                                    (self.ranges, *id),
                                    &def_name,
                                    def_type,
                                    call_expr_type
                                );
                            }
                        }

                        Ok(*return_type)
                    }
                } else {
                    msg!(Msg::FuncUndefined, (self.ranges, *id))
                }
            }
            Expr::VarName { name, id } => {
                if let Some(constant) = self.constants.get(name) {
                    Ok(*constant)
                } else if let Some(VarStore::Init(var_type)) = self.env.get(name) {
                    Ok(var_type)
                } else if let Some(VarStore::UnInit(_)) = self.env.get(name) {
                    msg!(Msg::AccessUninit, (self.ranges, *id), name)
                } else {
                    msg!(Msg::VarUndefined, (self.ranges, *id))
                }
            }
            Expr::TypeName { id, .. } => msg!(Msg::TypeEval, (self.ranges, *id)),
            Expr::TypeConversion { dtype, params, id } => {
                if let [to_convert] = params.as_slice() {
                    let original_type = self.expr_type(to_convert)?;
                    match (dtype, original_type) {
                        (Type::Int, Type::Char | Type::Bool | Type::Float) => Ok(Type::Int),
                        (Type::Float, Type::Int) => Ok(Type::Float),
                        (Type::Char, Type::Int) => Ok(Type::Char),
                        (Type::Bool, Type::Int) => Ok(Type::Bool),
                        // note sure if in the spec, but this seems reasonable...
                        (a, b) if a == &b => Ok(*a),
                        _ => msg!(Msg::TypeConvert, (self.ranges, *id)),
                    }
                } else {
                    msg!(Msg::ConvertAirty, (self.ranges, *id))
                }
            }

            Expr::Binary { lhs, op, rhs, id } => {
                let lhs_type = self.expr_type(lhs)?;
                let rhs_type = self.expr_type(rhs)?;

                if lhs_type != rhs_type {
                    msg!(Msg::TypeMatch, (self.ranges, *id))
                } else {
                    match op {
                        BinaryOp::Plus | BinaryOp::Divide | BinaryOp::Times | BinaryOp::Minus => {
                            if [Type::Float, Type::Int].contains(&lhs_type) {
                                Ok(lhs_type)
                            } else {
                                msg!(Msg::ExpectType, (self.ranges, *id), "int, float")
                            }
                        }
                        BinaryOp::Less
                        | BinaryOp::LessEqual
                        | BinaryOp::Greater
                        | BinaryOp::GreaterEqual => {
                            if [Type::Float, Type::Int, Type::Char].contains(&lhs_type) {
                                Ok(Type::Bool)
                            } else {
                                msg!(Msg::ExpectType, (self.ranges, *id), "int, float, char")
                            }
                        }
                        BinaryOp::EqualEqual | BinaryOp::NotEqual => Ok(Type::Bool),
                    }
                }
            }
            Expr::Grouping { e, .. } => Ok(self.expr_type(e)?),
            Expr::Logical { lhs, rhs, id, .. } => {
                let lhs_type = self.expr_type(lhs)?;
                let rhs_type = self.expr_type(rhs)?;

                if lhs_type != Type::Bool || rhs_type != Type::Bool {
                    msg!(Msg::ExpectType, (self.ranges, *id), "bool")
                } else {
                    Ok(Type::Bool)
                }
            }
            Expr::Unary { op, operand, id } => {
                let operand_type = self.expr_type(operand)?;
                match op {
                    UnaryOp::LogicalNot => {
                        if operand_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            msg!(Msg::ExpectType, (self.ranges, *id), "bool")
                        }
                    }
                    UnaryOp::Plus | UnaryOp::Minus => {
                        if [Type::Float, Type::Int].contains(&operand_type) {
                            Ok(operand_type)
                        } else {
                            msg!(Msg::ExpectType, (self.ranges, *id), "int, float")
                        }
                    }
                }
            }
            Expr::Literal { value, .. } => Ok(value.dtype()),
        }
    }
}
