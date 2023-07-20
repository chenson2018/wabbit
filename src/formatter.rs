use crate::ast::{Expr, Stmt};
use crate::operators::*;
use crate::WabbitType;
use std::fmt::Display;

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Print { value, .. } => write!(f, "print {value};"),
            Stmt::While {
                condition, body, ..
            } => {
                write!(f, "while {condition}{{")?;
                write!(f, "{body}")?;
                write!(f, "}}")
            }
            Stmt::Block { statements, .. } => {
                for stmt in statements {
                    write!(f, "{stmt}")?;
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_block,
                maybe_else_block,
                ..
            } => {
                write!(f, "if {condition}{{")?;
                write!(f, "{then_block}")?;
                write!(f, "}}")?;
                if let Some(else_block) = maybe_else_block {
                    write!(f, "else {{")?;
                    write!(f, "{else_block}")?;
                    write!(f, "}}")?;
                }
                Ok(())
            }
            Stmt::Expr(e) => {
                write!(f, "{e};")
            }
            Stmt::LoopControl { control, .. } => match control {
                LoopControl::Break => write!(f, "break;"),
                LoopControl::Continue => write!(f, "continue;"),
            },
            Stmt::FuncDef {
                def_name,
                def_params,
                return_type,
                body,
                ..
            } => {
                write!(f, "func {def_name}(")?;
                let airty = def_params.len();

                for (i, (name, typename)) in def_params.iter().enumerate() {
                    write!(f, "{name} {typename}")?;
                    if i != airty - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "){return_type}{{")?;
                write!(f, "{body}")?;
                write!(f, "}}")
            }
            Stmt::VarDef {
                name,
                maybe_type,
                maybe_value,
                ..
            } => match (maybe_type, maybe_value) {
                (Some(typename), Some(value)) => {
                    write!(f, "var {name} {typename}={value};")
                }
                (None, Some(value)) => {
                    write!(f, "var {name}={value};")
                }
                (Some(typename), None) => {
                    write!(f, "var {name} {typename};")
                }
                (None, None) => panic!(),
            },
            Stmt::ConstDef {
                name,
                maybe_type,
                value,
                ..
            } => {
                if let Some(typename) = maybe_type {
                    write!(f, "const {name} {typename}={value};")
                } else {
                    write!(f, "const {name}={value};")
                }
            }
            Stmt::Return { value, .. } => {
                write!(f, "return {value};")
            }
            Stmt::Assign { name, value, .. } => {
                write!(f, "{name}={value};")
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { value, .. } => match value {
                WabbitType::Char(c) if c == &'\n' => write!(f, "'\\n'"),
                WabbitType::Char(c) => write!(f, "'{c}'"),
                WabbitType::Float(x) => write!(f, "{x:.32}"),
                _ => write!(f, "{value}"),
            },
            Expr::Logical { lhs, op, rhs, .. } => {
                write!(f, "{lhs}{op}{rhs}")
            }
            Expr::Unary { op, operand, .. } => {
                write!(f, "{op}{operand}")
            }
            Expr::Binary { lhs, op, rhs, .. } => {
                write!(f, "{lhs}{op}{rhs}")
            }
            Expr::Call { name, params, .. } => {
                write!(f, "{name}(")?;

                let airty = params.len();

                for (i, e) in params.iter().enumerate() {
                    write!(f, "{e}")?;
                    if i != airty - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, ")")
            }
            Expr::TypeConversion { dtype, params, .. } => {
                write!(f, "{dtype}(")?;

                let airty = params.len();

                for (i, e) in params.iter().enumerate() {
                    write!(f, "{e}")?;
                    if i != airty - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::VarName { name, .. } => write!(f, "{name}"),
            Expr::TypeName { dtype, .. } => write!(f, "{dtype}"),
            Expr::Grouping { e, .. } => write!(f, "({e})"),
        }
    }
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalAnd => write!(f, "&&"),
            Self::LogicalOr => write!(f, "||"),
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Times => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::EqualEqual => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::LogicalNot => write!(f, "!"),
        }
    }
}
