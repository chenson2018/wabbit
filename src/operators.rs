use crate::error::{msg, Msg, RangeReporter, WabbitError};
use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    NotEqual,
}

impl TryFrom<Token> for BinaryOp {
    type Error = WabbitError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.lexeme.as_str() {
            "+" => Ok(BinaryOp::Plus),
            "-" => Ok(BinaryOp::Minus),
            "*" => Ok(BinaryOp::Times),
            "/" => Ok(BinaryOp::Divide),
            "<" => Ok(BinaryOp::Less),
            "<=" => Ok(BinaryOp::LessEqual),
            ">" => Ok(BinaryOp::Greater),
            ">=" => Ok(BinaryOp::GreaterEqual),
            "==" => Ok(BinaryOp::EqualEqual),
            "!=" => Ok(BinaryOp::NotEqual),
            _ => msg!(
                Msg::InternalErr,
                &token,
                "Parser allowed an invalid operator."
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
}

impl TryFrom<Token> for UnaryOp {
    type Error = WabbitError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.lexeme.as_str() {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "!" => Ok(Self::LogicalNot),
            _ => msg!(
                Msg::InternalErr,
                &token,
                "Parser allowed an invalid operator."
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    LogicalAnd,
    LogicalOr,
}

impl TryFrom<Token> for LogicalOp {
    type Error = WabbitError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.lexeme.as_str() {
            "&&" => Ok(Self::LogicalAnd),
            "||" => Ok(Self::LogicalOr),
            _ => msg!(
                Msg::InternalErr,
                &token,
                "Parser allowed an invalid operator."
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoopControl {
    Continue,
    Break,
}

impl TryFrom<Token> for LoopControl {
    type Error = WabbitError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.lexeme.as_str() {
            "break" => Ok(Self::Break),
            "continue" => Ok(Self::Continue),
            _ => msg!(
                Msg::InternalErr,
                &token,
                "Parser allowed an invalid loop control."
            ),
        }
    }
}
