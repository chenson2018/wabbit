use crate::tokens::Token;

use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

pub type Result<T> = std::result::Result<T, WabbitError>;
pub type Results<T> = std::result::Result<T, Vec<WabbitError>>;

/// an error, pointing to source code indices
#[derive(Debug, Clone)]
pub struct WabbitError {
    /// error text
    pub label: String,
    /// source code indices
    range: (usize, usize),
}

impl WabbitError {
    pub fn new<S>(label: &S, range: (usize, usize)) -> Self
    where
        S: Into<String> + std::fmt::Display,
    {
        Self {
            label: format!("\x1b[31m{label}\x1b[0m"),
            range,
        }
    }
}

/// struct for reporting multiple errors
#[derive(Debug, Clone)]
pub struct WabbitErrorReporter {
    /// vector of errors
    errors: Vec<WabbitError>,
    /// path to source file
    path: PathBuf,
    /// raw source code string
    source: String,
    /// stage at which error occurred
    title: String,
}

impl WabbitErrorReporter {
    pub fn new(errors: Vec<WabbitError>, path: PathBuf, source: String, title: &str) -> Self {
        Self {
            errors,
            path,
            source,
            title: title.into(),
        }
    }
}

impl Display for WabbitErrorReporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snip = Snippet {
            title: Some(Annotation {
                label: Some(&self.title),
                id: None,
                annotation_type: AnnotationType::Error,
            }),
            footer: vec![],
            slices: vec![Slice {
                source: &self.source,
                line_start: 1,
                origin: self.path.to_str(),
                fold: false,
                annotations: self
                    .errors
                    .iter()
                    .map(|WabbitError { label, range }| SourceAnnotation {
                        label,
                        range: *range,
                        annotation_type: AnnotationType::Error,
                    })
                    .collect(),
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        };
        let dl = DisplayList::from(snip);
        write!(f, "{dl}")
    }
}

impl Error for WabbitErrorReporter {}

/// trait for reporting the left and right source indices of an error
///
/// This trait is quite versatile, allowing error reporting from tokens, expressions,
/// parsers, typecheckers, and interpreters
pub trait RangeReporter {
    /// extract tokens from a general type
    fn extract_tokens<'a>(&'a self, output: &mut Vec<&'a Token>);

    /// use `extract_tokens` to get tokens, then find the left and right edge
    fn extract_range(&self) -> (usize, usize) {
        let mut tokens = Vec::new();
        self.extract_tokens(&mut tokens);
        let ranges: Vec<&usize> = tokens
            .iter()
            .flat_map(
                |Token {
                     range: (left, right),
                     ..
                 }| [left, right],
            )
            .collect();
        (
            **ranges.iter().min().unwrap_or(&&0),
            **ranges.iter().max().unwrap_or(&&0),
        )
    }
}

/// convenience implementation for a pair of tokens/expressions/statements
///
/// this assumes that we are given items in order from left to right

impl<L, R> RangeReporter for (L, R)
where
    L: RangeReporter,
    R: RangeReporter,
{
    fn extract_tokens<'a>(&'a self, _output: &mut Vec<&'a Token>) {}

    fn extract_range(&self) -> (usize, usize) {
        let (t1, t2) = self;
        let (left, _) = t1.extract_range();
        let (_, right) = t2.extract_range();
        (left, right)
    }
}

/// collection of various errors that could be raised by an ill-formed Wabbit program
///
/// the one exception to this is `Msg::InternalErr`, which reports a failure of the crate itself
///
/// see [`Msg::msg`] for the messages presented to the user

#[derive(Clone, Copy)]
pub enum Msg {
    // Scanner
    InvalidNumber,
    InvalidChar,
    DoubleToken,
    UnexpectedChar,

    // Parser
    VarDefEmpty,
    ExpectExpr,
    ExpectTypeName,
    ExpectVarName,
    ParserExpect,

    // Typechecker/Interpreter
    AltBranch,
    DupArgs,
    FuncDefScope,
    ReturnScope,
    AssignRetype,
    VarUndefined,
    AssignUndefined,
    ConstScope,
    InitType,
    LoopReq,
    TypeConvert,
    ConvertAirty,
    FuncAirty,
    ParamType,
    ReturnType,
    NoReturn,
    FuncUndefined,
    TypeEval,
    AccessUninit,
    ExpectType,
    TypeMatch,
    ReturnDiverge,
    RedeclareVar,
    RedeclareFunc,
    RedeclareConst,

    // errors that are NOT user errors
    InternalErr,
}

impl Msg {
    /// an error message template
    pub fn msg(&self) -> &'static str {
        match self {
            // Scanner
            Msg::InvalidNumber => "invalid number: '{}'",
            Msg::InvalidChar => "invalid character",
            Msg::DoubleToken => "character '{}' is invalid, maybe you meant to follow with '{}'?",
            Msg::UnexpectedChar => "unexpected character '{}'",

            // Parser
            Msg::VarDefEmpty => "variable definitions must contain either a type or expression.",
            Msg::ExpectExpr => "expected an expression",
            Msg::ExpectTypeName => "expected a type name (int, float, bool, or char)",
            Msg::ExpectVarName => "expected a variable name",
            Msg::ParserExpect => "expected '{}'",

            // Typechecker/Interpreter
            Msg::AltBranch => "some branches do not have a return value",
            Msg::DupArgs => "function arguments must have unique names",
            Msg::FuncDefScope => "functions must be declared in the global scope",
            Msg::ConstScope => "constants must be declared in global scope",
            Msg::ReturnScope => "must be inside a function",
            Msg::LoopReq => "must be inside a while block",
            Msg::AssignRetype => {
                "'{}' previously defined with type '{}', cannot assign a new value with type '{}'"
            }
            Msg::VarUndefined => "undefined variable",
            Msg::FuncUndefined => "undefined function",
            Msg::AssignUndefined => "assignment to undefined variable",
            Msg::InitType => "initial value and type declaration do not match",
            Msg::TypeConvert => "invalid type conversion",
            Msg::ConvertAirty => "type conversions take a single argument",
            Msg::FuncAirty => "'{}' defined with {} parameters, but called with {}",
            Msg::ParamType => "parameter '{}' defined with type '{}', but called with type '{}'",
            Msg::ReturnType => "'{}' defined with return type '{}', but returned type '{}'",
            Msg::NoReturn => "function did not return a value",
            Msg::TypeEval => "type names cannot be used as values",
            Msg::AccessUninit => "cannot read uninitialized variable, {}",
            Msg::ExpectType => "expected types: {}",
            Msg::TypeMatch => "differing argument types",
            Msg::ReturnDiverge => "multiple return types",
            Msg::RedeclareVar => "'{}' is previously declared as a variable",
            Msg::RedeclareConst => "'{}' is previously declared as a constant",
            Msg::RedeclareFunc => "'{}' is previously declared as a function",

            // errors that are NOT user errors
            Msg::InternalErr => "This is an internal error! {}",
        }
    }
}

/// convenience macro for raising errors
macro_rules! msg {
    ($code: expr, $e: expr $(, $args:expr)*) => {{
        let args: &[String] = &[ $($args.to_string()),* ];
        let msg = <dynfmt::SimpleCurlyFormat as dynfmt::Format>::format(&dynfmt::SimpleCurlyFormat, $code.msg(), args).expect("error formatting failed");
        Err(crate::error::WabbitError::new(&msg, $e.extract_range()))
    }};
}

/// convenience macro for construction of a `WabbitError`
macro_rules! err {
    ($code: expr, $e: expr $(, $args:expr)*) => {{
        let args: &[String] = &[ $($args.to_string()),* ];
        let msg = <dynfmt::SimpleCurlyFormat as dynfmt::Format>::format(&dynfmt::SimpleCurlyFormat, $code.msg(), args).expect("error formatting failed");
        crate::error::WabbitError::new(&msg, $e.extract_range())
    }};
}

pub(crate) use err;
pub(crate) use msg;
