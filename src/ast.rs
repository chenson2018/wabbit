use crate::types::Type;
use crate::{operators::*, WabbitType};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Print {
        value: Expr,
        id: usize,
    },
    VarDef {
        name: String,
        maybe_type: Option<Type>,
        maybe_value: Option<Expr>,
        id: usize,
    },
    ConstDef {
        name: String,
        maybe_type: Option<Type>,
        value: Expr,
        id: usize,
    },
    FuncDef {
        def_name: String,
        def_params: Vec<(String, Type)>,
        return_type: Type,
        body: Box<Stmt>,
        id: usize,
    },
    If {
        condition: Expr,
        then_block: Box<Stmt>,
        maybe_else_block: Option<Box<Stmt>>,
        id: usize,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
        id: usize,
    },
    LoopControl {
        control: LoopControl,
        id: usize,
    },
    Return {
        value: Expr,
        id: usize,
    },
    Assign {
        name: String,
        value: Expr,
        id: usize,
    },
    Block {
        statements: Vec<Stmt>,
        id: usize,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Call {
        name: String,
        params: Vec<Expr>,
        id: usize,
    },
    TypeConversion {
        dtype: Type,
        params: Vec<Expr>,
        id: usize,
    },
    Logical {
        lhs: Box<Expr>,
        op: LogicalOp,
        rhs: Box<Expr>,
        id: usize,
    },
    VarName {
        name: String,
        id: usize,
    },
    TypeName {
        dtype: Type,
        id: usize,
    },
    Grouping {
        e: Box<Expr>,
        id: usize,
    },
    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
        id: usize,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        id: usize,
    },
    Literal {
        value: WabbitType,
        id: usize,
    },
}
