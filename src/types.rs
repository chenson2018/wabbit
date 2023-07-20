use std::fmt::Display;

/// Wabbit data types

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Type {
    Int,
    Char,
    Bool,
    Float,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
        }
    }
}

/// a Wabbit value, including Wabbit types

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WabbitType {
    Int(i32),
    Float(f64),
    Char(char),
    Bool(bool),
    TypeHolder(Type),
}

impl From<char> for WabbitType {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl From<f64> for WabbitType {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for WabbitType {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for WabbitType {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl Display for WabbitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Bool(val) => write!(f, "{val}"),
            Self::Int(val) => write!(f, "{val}"),
            Self::Char(val) => write!(f, "{val}"),
            Self::Float(val) => write!(f, "{val}"),
            Self::TypeHolder(val) => write!(f, "{val}"),
        }
    }
}

impl WabbitType {
    pub fn dtype(&self) -> Type {
        match self {
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Char(_) => Type::Char,
            Self::Float(_) => Type::Float,
            Self::TypeHolder(t) => *t,
        }
    }

    pub fn bool_compare(self, other: WabbitType, f: impl Fn(bool, bool) -> bool) -> WabbitType {
        match (self, other) {
            (WabbitType::Bool(a), WabbitType::Bool(b)) => WabbitType::Bool(f(a, b)),
            _ => panic!("Invalid arguments to bool_compare"),
        }
    }

    pub fn char_compare(self, other: WabbitType, f: impl Fn(char, char) -> bool) -> WabbitType {
        match (self, other) {
            (WabbitType::Char(a), WabbitType::Char(b)) => WabbitType::Bool(f(a, b)),
            _ => panic!("Invalid arguments to char_compare"),
        }
    }

    pub fn float_compare(self, other: WabbitType, f: impl Fn(f64, f64) -> bool) -> WabbitType {
        match (self, other) {
            (WabbitType::Float(a), WabbitType::Float(b)) => WabbitType::Bool(f(a, b)),
            _ => panic!("Invalid arguments to float_compare"),
        }
    }

    pub fn int_compare(self, other: WabbitType, f: impl Fn(i32, i32) -> bool) -> WabbitType {
        match (self, other) {
            (WabbitType::Int(a), WabbitType::Int(b)) => WabbitType::Bool(f(a, b)),
            _ => panic!("Invalid arguments to int_compare"),
        }
    }

    pub fn float_binary(self, other: WabbitType, f: impl Fn(f64, f64) -> f64) -> WabbitType {
        match (self, other) {
            (WabbitType::Float(a), WabbitType::Float(b)) => WabbitType::Float(f(a, b)),
            _ => panic!("Invalid arguments to float_binary"),
        }
    }

    pub fn int_binary(self, other: WabbitType, f: impl Fn(i32, i32) -> i32) -> WabbitType {
        match (self, other) {
            (WabbitType::Int(a), WabbitType::Int(b)) => WabbitType::Int(f(a, b)),
            _ => panic!("Invalid arguments to int_binary"),
        }
    }

    pub fn float_unary(self, f: impl Fn(f64) -> f64) -> WabbitType {
        match self {
            WabbitType::Float(a) => WabbitType::Float(f(a)),
            _ => panic!("Invalid arguments to float_unary"),
        }
    }

    pub fn int_unary(self, f: impl Fn(i32) -> i32) -> WabbitType {
        match self {
            WabbitType::Int(a) => WabbitType::Int(f(a)),
            _ => panic!("Invalid arguments to int_unary"),
        }
    }
}

macro_rules! numeric_unary {
    ($op:ident, $loc:expr, $closure:tt) => {
        match $op {
            WabbitType::Int(_) => Ok($op.int_unary($closure)),
            WabbitType::Float(_) => Ok($op.float_unary($closure)),
            _ => msg!(Msg::ExpectType, $loc, "int, float"),
        }
    };
}

macro_rules! numeric_binary {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitType::Int(_), WabbitType::Int(_)) => Ok($op1.int_binary($op2, |a, b| a $op b)),
            (WabbitType::Float(_), WabbitType::Float(_)) => Ok($op1.float_binary($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float")
        }
    };
}

macro_rules! compare {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitType::Int(_), WabbitType::Int(_)) => Ok($op1.int_compare($op2, |a, b| a $op b)),
            (WabbitType::Float(_), WabbitType::Float(_)) => Ok($op1.float_compare($op2, |a, b| a $op b)),
            (WabbitType::Char(_), WabbitType::Char(_)) => Ok($op1.char_compare($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float, char")
        }
    };
}

macro_rules! equality {
    ($op1:ident, $op2:ident, $loc:expr, $op:tt) => {
        match (&$op1, &$op2) {
            (WabbitType::Int(_), WabbitType::Int(_)) => Ok($op1.int_compare($op2, |a, b| a $op b)),
            (WabbitType::Float(_), WabbitType::Float(_)) => Ok($op1.float_compare($op2, |a, b| a $op b)),
            (WabbitType::Char(_), WabbitType::Char(_)) => Ok($op1.char_compare($op2, |a, b| a $op b)),
            (WabbitType::Bool(_), WabbitType::Bool(_)) => Ok($op1.bool_compare($op2, |a, b| a $op b)),
            _ => msg!(Msg::ExpectType, $loc, "int, float, char, bool")
        }
    };
}

pub(crate) use {compare, equality, numeric_binary, numeric_unary};
