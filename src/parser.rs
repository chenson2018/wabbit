use crate::ast::{Expr, Stmt};
use crate::error::{msg, Msg, RangeReporter, Result};
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType};
use crate::types::Type;
use std::collections::HashMap;

/// Struct for transforming tokens into a vector of statements (AST)

#[derive(Debug)]
pub struct Parser<'a> {
    /// index of `self.tokens` that the parser is examining
    current: usize,
    /// tokens, borrowed from a scanner
    tokens: &'a Vec<Token>,
    /// resulting statements after parsing
    statements: Vec<Stmt>,
    /// current expression/statement index
    id: usize,
    /// a map from statement/expression indices to source indices
    ranges: HashMap<usize, (usize, usize)>,
}

impl<'a> From<&'a Scanner> for Parser<'a> {
    fn from(scanner: &'a Scanner) -> Self {
        let tokens = scanner.borrow_tokens();
        Parser::new(tokens)
    }
}

/// given a parser, can copy the range of the current token
impl RangeReporter for Parser<'_> {
    fn extract_tokens<'a>(&'a self, _output: &mut Vec<&'a Token>) {}

    fn extract_range(&self) -> (usize, usize) {
        self.peek().range
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            current: 0,
            statements: Vec::new(),
            tokens,
            id: 0,
            ranges: HashMap::new(),
        }
    }

    /// given a pair of tokens/expressions/statements, record the furthest left and right source indices
    fn assign_id<L, R>(&mut self, left: L, right: R) -> usize
    where
        L: RangeReporter,
        R: RangeReporter,
    {
        self.id += 1;
        self.ranges.insert(self.id, (left, right).extract_range());
        self.id
    }

    /// record the source range of a single token
    fn assign_id_single(&mut self, token: &Token) -> usize {
        self.id += 1;
        self.ranges.insert(self.id, token.extract_range());
        self.id
    }

    /// return a reference to `self.statements`
    pub fn borrow_statements(&self) -> &Vec<Stmt> {
        &self.statements
    }

    /// return a reference to `self.ranges`
    pub fn borrow_ranges(&self) -> &HashMap<usize, (usize, usize)> {
        &self.ranges
    }

    /// parse all statements
    pub fn parse(&mut self) -> Result<()> {
        while !self.is_end() {
            let stmt = self.statement()?;
            self.statements.push(stmt);
        }
        Ok(())
    }

    /// parse a single statement
    ///
    /// This is the entry point for the mutually recursive private functions found below.
    fn statement(&mut self) -> Result<Stmt> {
        match self.advance().token {
            TokenType::LeftBrace => self.block(),
            TokenType::Var => self.vardef(),
            TokenType::Name => self.stmt_name(),
            TokenType::Const => self.constdef(),
            TokenType::Func => self.funcdef(),
            TokenType::Continue | TokenType::Break => self.loop_control(),
            TokenType::While => self.while_stmt(),
            TokenType::If => self.if_stmt(),
            TokenType::Print => self.print_stmt(),
            TokenType::Return => self.return_stmt(),
            _ => {
                self.current -= 1;
                self.expr_stmt()
            }
        }
    }

    /// return the current token without advancing the parser
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// return the previous token (or current if at the first token)
    fn previous(&mut self) -> Token {
        if self.current == 0 {
            self.peek()
        } else {
            self.tokens[self.current - 1].clone()
        }
    }

    /// check if all statements have been parsed
    fn is_end(&mut self) -> bool {
        self.peek().token == TokenType::Eof
    }

    /// return the current token and advance the parser one token
    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// check if the current token matches
    fn check(&mut self, t: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        self.peek().token == t
    }

    /// check if any token is a match, and if so advance
    fn match_any<T>(&mut self, types: T) -> bool
    where
        T: IntoIterator<Item = TokenType>,
    {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// advance past and return a specific token, or return an error
    fn expect(&mut self, c: char) -> Result<Token> {
        let tt = match c {
            '=' => TokenType::Assign,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            ';' => TokenType::Semicolon,
            _ => return msg!(Msg::InternalErr, self, "Unexpected char input to expect"),
        };
        if self.check(tt) {
            Ok(self.advance())
        } else {
            msg!(Msg::ParserExpect, self, c)
        }
    }

    /// advance past and return a name, or return an error
    fn get_name(&mut self) -> Result<Token> {
        if self.match_any([TokenType::Name]) {
            let var_name = self.previous();
            Ok(var_name)
        } else {
            msg!(Msg::ExpectVarName, self)
        }
    }

    /// advance past and return a type, or return an error
    fn get_type(&mut self) -> Result<(Token, Type)> {
        if self.match_any([
            TokenType::IntegerType,
            TokenType::FloatType,
            TokenType::CharType,
            TokenType::BoolType,
        ]) {
            let dtype = match self.previous().token {
                TokenType::IntegerType => Type::Int,
                TokenType::FloatType => Type::Float,
                TokenType::CharType => Type::Char,
                TokenType::BoolType => Type::Bool,
                _ => {
                    return msg!(
                        Msg::InternalErr,
                        self,
                        "Parser attempted to consruct a type from a non-type token"
                    )
                }
            };
            Ok((self.previous(), dtype))
        } else {
            msg!(Msg::ExpectTypeName, self)
        }
    }

    fn block(&mut self) -> Result<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        let lead = self.previous();
        while !self.check(TokenType::RightBrace) && !self.is_end() {
            statements.push(self.statement()?);
        }
        let last = self.expect('}')?;
        let id = self.assign_id(&lead, &last);

        Ok(Stmt::Block { statements, id })
    }

    fn vardef(&mut self) -> Result<Stmt> {
        let name = self.get_name()?;
        let mut count = 0;

        let maybe_type = if let Ok((_, type_name)) = self.get_type() {
            count += 1;
            Some(type_name)
        } else {
            None
        };

        let maybe_value = if self.check(TokenType::Assign) {
            count += 1;
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };

        if count == 0 {
            msg!(Msg::VarDefEmpty, self)
        } else {
            let last = self.expect(';')?;
            let id = self.assign_id(&name, &last);

            Ok(Stmt::VarDef {
                name: name.lexeme,
                maybe_type,
                maybe_value,
                id,
            })
        }
    }

    fn stmt_name(&mut self) -> Result<Stmt> {
        if self.check(TokenType::LeftParen) {
            self.current -= 1;
            let e = self.expression()?;
            self.expect(';')?;
            Ok(Stmt::Expr(e))
        } else if self.check(TokenType::Assign) {
            let name = self.previous();
            self.advance();
            let value = self.expression()?;
            let last = self.expect(';')?;
            let id = self.assign_id(&name, &last);
            Ok(Stmt::Assign {
                name: name.lexeme,
                value,
                id,
            })
        } else {
            self.current -= 1;
            Ok(self.expr_stmt()?)
        }
    }

    fn constdef(&mut self) -> Result<Stmt> {
        let name = self.get_name()?;
        let maybe_type = if let Ok((_, type_name)) = self.get_type() {
            Some(type_name)
        } else {
            None
        };
        self.expect('=')?;
        let value = self.expression()?;
        let last = self.expect(';')?;

        let id = self.assign_id(&name, &last);

        Ok(Stmt::ConstDef {
            name: name.lexeme,
            maybe_type,
            value,
            id,
        })
    }

    fn funcdef(&mut self) -> Result<Stmt> {
        let lead = self.previous();
        let def_name = self.get_name()?.lexeme;
        let mut def_params: Vec<(String, Type)> = Vec::new();

        self.expect('(')?;

        while !self.check(TokenType::RightParen) {
            let var_name = self.get_name()?.lexeme;
            let (_, type_name) = self.get_type()?;
            def_params.push((var_name, type_name));
            if self.check(TokenType::Comma) {
                self.advance();
            }
        }

        self.expect(')')?;
        let (_, return_type) = self.get_type()?;
        self.expect('{')?;
        let body = box self.block()?;
        let last = self.previous();
        let id = self.assign_id(&lead, &last);

        Ok(Stmt::FuncDef {
            def_name,
            def_params,
            return_type,
            body,
            id,
        })
    }

    fn loop_control(&mut self) -> Result<Stmt> {
        let token = self.previous();
        let last = self.expect(';')?;
        let id = self.assign_id(&token, &last);

        Ok(Stmt::LoopControl {
            control: token.try_into()?,
            id,
        })
    }

    fn while_stmt(&mut self) -> Result<Stmt> {
        let lead = self.previous();
        let condition = self.expression()?;
        self.expect('{')?;
        let body = box self.block()?;
        let last = self.previous();
        let id = self.assign_id(&lead, &last);

        Ok(Stmt::While {
            condition,
            body,
            id,
        })
    }

    fn if_stmt(&mut self) -> Result<Stmt> {
        let lead = self.previous();
        let condition = self.expression()?;
        self.expect('{')?;
        let then_block = box self.block()?;

        let maybe_else_block = if self.match_any([TokenType::Else]) {
            self.expect('{')?;
            let else_block = box self.block()?;
            Some(else_block)
        } else {
            None
        };

        let last = self.previous();
        let id = self.assign_id(&lead, &last);

        Ok(Stmt::If {
            condition,
            then_block,
            maybe_else_block,
            id,
        })
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        let lead = self.previous();
        let value = self.expression()?;
        let last = self.expect(';')?;
        let id = self.assign_id(&lead, &last);
        Ok(Stmt::Print { value, id })
    }

    fn return_stmt(&mut self) -> Result<Stmt> {
        let lead = self.previous();
        let value = self.expression()?;
        let last = self.expect(';')?;
        let id = self.assign_id(&lead, &last);
        Ok(Stmt::Return { value, id })
    }

    // an expression appearing on its own, with value unused
    fn expr_stmt(&mut self) -> Result<Stmt> {
        let e = self.expression()?;
        self.expect(';')?;
        Ok(Stmt::Expr(e))
    }

    // These functions all descend to define an expression

    fn expression(&mut self) -> Result<Expr> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr> {
        let lead = self.previous();
        let mut e = self.and()?;

        while self.match_any([TokenType::LogicalOr]) {
            let op = self.previous();
            let rhs = box self.and()?;
            let last = self.previous();
            let id = self.assign_id(&lead, &last);
            e = Expr::Logical {
                lhs: box e,
                op: op.try_into()?,
                rhs,
                id,
            };
        }
        Ok(e)
    }

    fn and(&mut self) -> Result<Expr> {
        let lead = self.previous();
        let mut e = self.compare()?;

        while self.match_any([TokenType::LogicalAnd]) {
            let op = self.previous();
            let rhs = box self.and()?;
            let last = self.previous();
            let id = self.assign_id(&lead, &last);
            e = Expr::Logical {
                lhs: box e,
                op: op.try_into()?,
                rhs,
                id,
            };
        }
        Ok(e)
    }

    fn compare(&mut self) -> Result<Expr> {
        let lead = self.previous();
        let mut e = self.add_or_sub()?;

        while self.match_any([
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::EqualEqual,
            TokenType::NotEqual,
        ]) {
            let op = self.previous();
            let rhs = box self.add_or_sub()?;
            let last = self.previous();
            let id = self.assign_id(&lead, &last);
            e = Expr::Binary {
                lhs: box e,
                op: op.try_into()?,
                rhs,
                id,
            };
        }
        Ok(e)
    }

    fn add_or_sub(&mut self) -> Result<Expr> {
        let lead = self.previous();
        let mut e = self.times_or_div()?;

        while self.match_any([TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let rhs = box self.times_or_div()?;
            let last = self.previous();
            let id = self.assign_id(&lead, &last);
            e = Expr::Binary {
                lhs: box e,
                op: op.try_into()?,
                rhs,
                id,
            };
        }
        Ok(e)
    }

    fn times_or_div(&mut self) -> Result<Expr> {
        let lead = self.previous();
        let mut e = self.unary()?;

        while self.match_any([TokenType::Divide, TokenType::Times]) {
            let op = self.previous();
            let rhs = box self.unary()?;
            let last = self.previous();
            let id = self.assign_id(&lead, &last);
            e = Expr::Binary {
                lhs: box e,
                op: op.try_into()?,
                rhs,
                id,
            };
        }

        Ok(e)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_any([TokenType::Minus, TokenType::Plus, TokenType::LogicalNot]) {
            let op = self.previous();
            let operand = box self.unary()?;
            let last = self.previous();
            let id = self.assign_id(&op, &last);
            Ok(Expr::Unary {
                op: op.try_into()?,
                operand,
                id,
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut e = self.primary()?;
        if self.match_any([TokenType::LeftParen]) {
            let name = self.previous();
            e = self.finish_call(e, &name)?;
        }
        Ok(e)
    }

    fn finish_call(&mut self, e: Expr, lead: &Token) -> Result<Expr> {
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            params.push(self.expression()?);
            while self.match_any([TokenType::Comma]) {
                params.push(self.expression()?); // the name
            }
        }
        let last = self.expect(')')?;
        let id = self.assign_id(lead, &last);
        match e {
            Expr::VarName { name, .. } => Ok(Expr::Call { name, params, id }),
            Expr::TypeName { dtype, .. } => Ok(Expr::TypeConversion { dtype, params, id }),
            _ => msg!(
                Msg::InternalErr,
                self,
                "Parser constructed call without a name/type"
            ),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_any([
            TokenType::False,
            TokenType::True,
            TokenType::Float,
            TokenType::Integer,
            TokenType::Char,
        ]) {
            let token = self.previous();
            let id = self.assign_id_single(&token);
            if let Some(value) = token.literal {
                Ok(Expr::Literal { value, id })
            } else {
                msg!(
                    Msg::InternalErr,
                    &token,
                    "Scanner created a literal without a value."
                )
            }
        } else if let Ok((token, dtype)) = self.get_type() {
            let id = self.assign_id_single(&token);
            Ok(Expr::TypeName { id, dtype })
        } else if let Ok(name) = self.get_name() {
            let id = self.assign_id_single(&name);
            Ok(Expr::VarName {
                name: name.lexeme,
                id,
            })
        } else if self.match_any([TokenType::LeftParen]) {
            let lead = self.previous();
            let e = box self.expression()?;
            let last = self.expect(')')?;
            let id = self.assign_id(&lead, &last);
            Ok(Expr::Grouping { e, id })
        } else {
            msg!(Msg::ExpectExpr, self)
        }
    }
}
