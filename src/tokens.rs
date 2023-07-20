use crate::error::RangeReporter;
use crate::types::WabbitType;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Const,
    Var,
    Print,
    Break,
    Continue,
    If,
    Else,
    While,
    Func,
    Return,
    True,
    False,

    // Identifiers/Names
    Name,
    Type,

    // Built-in Types
    IntegerType,
    FloatType,
    CharType,
    BoolType,

    // literals
    Char,
    Integer,
    Float,

    // Symbols and operators
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
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Assign,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    // end of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub literal: Option<WabbitType>,
    pub line: usize,
    pub range: (usize, usize),
}

/// given a reference to a token, can copy its range
impl RangeReporter for &Token {
    fn extract_tokens<'a>(&'a self, _output: &mut Vec<&'a Token>) {}

    fn extract_range(&self) -> (usize, usize) {
        self.range
    }
}
