use crate::ast::Stmt;
use crate::environment::Environment;
use crate::error::{msg, Msg, RangeReporter, Result};
use crate::parser::Parser;
use std::collections::HashMap;

/// struct for typechecking or interpreting Wabbit AST

#[derive(Debug)]
pub struct Analyzer<'a, T>
where
    T: Clone,
{
    /// statements, borrowed from a parser
    pub(crate) statements: &'a Vec<Stmt>,
    /// map of variables to types or values
    pub(crate) env: Environment<'a, T>,
    /// map of constants to types or values
    pub(crate) constants: HashMap<&'a String, T>,
    /// map of functions to types or values
    pub(crate) functions: HashMap<&'a String, &'a Stmt>,
    /// record of IO (print statements)
    pub output: Vec<T>,
    /// current depth of function calls
    pub(crate) call_depth: usize,
    /// current depth of while loops
    pub(crate) loop_depth: usize,
    /// map of expressions/statemts to source indices, borrowed from a parser
    pub(crate) ranges: &'a HashMap<usize, (usize, usize)>,
}

/// this implementation allows reporting errors from any type that stores a map from expressions/statemts to source indices

impl RangeReporter for (&HashMap<usize, (usize, usize)>, &usize) {
    fn extract_tokens<'a>(&'a self, _output: &mut Vec<&'a crate::tokens::Token>) {}

    fn extract_range(&self) -> (usize, usize) {
        let (map, id) = self;
        match map.get(id) {
            Some(loc) => *loc,
            None => panic!("Analyzer tried to report invalid location id {id}."),
        }
    }
}

impl<'a, T> Analyzer<'a, T>
where
    T: Clone,
{
    pub fn new(statements: &'a Vec<Stmt>, ranges: &'a HashMap<usize, (usize, usize)>) -> Self {
        Self {
            env: Environment::new(),
            constants: HashMap::new(),
            functions: HashMap::new(),
            output: Vec::new(),
            statements,
            call_depth: 0,
            loop_depth: 0,
            ranges,
        }
    }

    /// check if a name is already used by a variable name in the current scope
    pub(crate) fn check_env(&self, name: &String, id: &usize) -> Result<()> {
        if self.env.top_contains(name) {
            msg!(Msg::RedeclareVar, (self.ranges, id), name)
        } else {
            Ok(())
        }
    }

    /// check if a name is already used by a constant
    pub(crate) fn check_constant(&self, name: &String, id: &usize) -> Result<()> {
        if self.constants.contains_key(name) {
            msg!(Msg::RedeclareConst, (self.ranges, id), name)
        } else {
            Ok(())
        }
    }

    /// check if a name is already used by a function
    pub(crate) fn check_function(&self, name: &String, id: &usize) -> Result<()> {
        if self.functions.contains_key(name) {
            msg!(Msg::RedeclareFunc, (self.ranges, id), name)
        } else {
            Ok(())
        }
    }
}

impl<'a, T> From<&'a Parser<'a>> for Analyzer<'a, T>
where
    T: Clone,
{
    fn from(parser: &'a Parser) -> Self {
        let statements = parser.borrow_statements();
        let ranges = parser.borrow_ranges();
        Self::new(statements, ranges)
    }
}
