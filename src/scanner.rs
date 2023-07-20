use crate::error::{err, msg, Msg, RangeReporter, Result, Results};
use crate::tokens::{Token, TokenType};
use crate::types::WabbitType;
use std::collections::HashMap;

lazy_static! {
    static ref TOKENS_SINGLE: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('+', TokenType::Plus);
        m.insert('-', TokenType::Minus);
        m.insert('*', TokenType::Times);
        m.insert(';', TokenType::Semicolon);
        m.insert('(', TokenType::LeftParen);
        m.insert(')', TokenType::RightParen);
        m.insert('{', TokenType::LeftBrace);
        m.insert('}', TokenType::RightBrace);
        m.insert(',', TokenType::Comma);
        m
    };
    static ref TOKENS_DOUBLE: HashMap<char, (char, Option<TokenType>, TokenType)> = {
        let mut m = HashMap::new();
        m.insert('<', ('=', Some(TokenType::Less), TokenType::LessEqual));
        m.insert(
            '>',
            ('=', Some(TokenType::Greater), TokenType::GreaterEqual),
        );
        m.insert('=', ('=', Some(TokenType::Assign), TokenType::EqualEqual));
        m.insert('!', ('=', Some(TokenType::LogicalNot), TokenType::NotEqual));
        m.insert('&', ('&', None, TokenType::LogicalAnd));
        m.insert('|', ('|', None, TokenType::LogicalOr));
        m
    };
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("const", TokenType::Const);
        m.insert("var", TokenType::Var);
        m.insert("print", TokenType::Print);
        m.insert("break", TokenType::Break);
        m.insert("continue", TokenType::Continue);
        m.insert("if", TokenType::If);
        m.insert("else", TokenType::Else);
        m.insert("while", TokenType::While);
        m.insert("func", TokenType::Func);
        m.insert("return", TokenType::Return);
        m.insert("true", TokenType::True);
        m.insert("false", TokenType::False);
        m
    };
    static ref TYPES: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("int", TokenType::IntegerType);
        m.insert("float", TokenType::FloatType);
        m.insert("bool", TokenType::BoolType);
        m.insert("char", TokenType::CharType);
        m
    };
}

/// Struct for transforming the raw character input of a Wabbit program into tokens

#[derive(Debug)]
pub struct Scanner {
    /// raw character input of a Wabbit program
    source: Vec<char>,
    /// resulting tokens after scanning `self.source`
    tokens: Vec<Token>,
    /// index of `self.source` that the scanner is examining
    current: usize,
    /// current line number the scanner is examining
    line: usize,
    /// starting index before scanner gets next token
    start: usize,
}

/// given a scanner, can copy the range it was currently processing
impl RangeReporter for Scanner {
    fn extract_tokens<'a>(&'a self, _output: &mut Vec<&'a Token>) {}

    fn extract_range(&self) -> (usize, usize) {
        (self.start, self.current)
    }
}

impl Scanner {
    /// get a reference to a scanner's tokens
    pub fn borrow_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// initialize a new scanner
    pub fn new(s: String) -> Scanner {
        Scanner {
            source: s.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
            current: 0,
            line: 0,
            start: 0,
        }
    }

    /// return the current character and advance the scanner one character
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    /// get the current range of token characters as a string
    fn lexeme(&self) -> String {
        self.source[self.start..self.current]
            .iter()
            .collect::<String>()
    }

    /// add a non-literal token to `self.tokens`
    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(Token {
            token,
            lexeme: self.lexeme(),
            line: self.line,
            literal: None,
            range: (self.start, self.current),
        });
    }

    /// add a literal token (string, number, or identifier) to `self.tokens`
    fn add_literal_token(&mut self, token: TokenType, l: WabbitType) {
        self.tokens.push(Token {
            token,
            lexeme: self.lexeme(),
            line: self.line,
            literal: Some(l),
            range: (self.start, self.current),
        });
    }

    /// check if all characters have been scanned
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// return the current character without advancing the scanner
    fn peek(&mut self) -> char {
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// return the following character without advancing the scanner
    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// scan an identifier
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let binding = self.lexeme();
        let lexeme = binding.as_str();

        if let Some(keyword_tt) = KEYWORDS.get(&lexeme) {
            match keyword_tt {
                TokenType::False => {
                    self.add_literal_token(TokenType::False, WabbitType::Bool(false))
                }
                TokenType::True => self.add_literal_token(TokenType::True, WabbitType::Bool(true)),
                _ => self.add_token(keyword_tt.clone()),
            }
        } else if let Some(type_tt) = TYPES.get(&lexeme) {
            self.add_token(type_tt.clone())
        } else {
            self.add_token(TokenType::Name)
        };
    }

    /// scan a numeric literal (integer or float)

    fn number(&mut self, mut found_decimal: bool) -> Result<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            found_decimal = true;
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme = self.lexeme();

        if found_decimal {
            match lexeme.parse::<f64>() {
                Ok(n) => self.add_literal_token(TokenType::Float, WabbitType::Float(n)),
                Err(_) => return msg!(Msg::InvalidNumber, self, lexeme),
            }
        } else {
            match lexeme.parse::<i32>() {
                Ok(n) => self.add_literal_token(TokenType::Integer, WabbitType::Int(n)),
                Err(_) => return msg!(Msg::InvalidNumber, self, lexeme),
            }
        }

        Ok(())
    }

    /// scan a single token

    fn scan_token(&mut self) -> Results<()> {
        let mut errors = Vec::new();
        let mut failed = false;
        let c = self.advance();

        // NOTE I assume unix line endings

        if self.is_end() {
            self.add_token(TokenType::Eof)
        } else if let Some(single_tt) = TOKENS_SINGLE.get(&c) {
            self.add_token(single_tt.clone())
        } else if let Some((next_for_double, maybe_single, double_tt)) = TOKENS_DOUBLE.get(&c) {
            // checking for pairs of chars that make a token
            if self.peek() == *next_for_double {
                self.advance();
                self.add_token(double_tt.clone())
            } else if let Some(single_tt) = maybe_single {
                self.add_token(single_tt.clone())
            } else {
                failed = true;
                let err = err!(Msg::DoubleToken, self, c, next_for_double);
                errors.push(err);
            };
        } else {
            match c {
                '/' => {
                    // handle single line comments
                    if self.peek() == '/' {
                        while self.peek() != '\n' && !(self.is_end()) {
                            self.advance();
                        }
                    } else if self.peek() == '*' {
                        loop {
                            let next = self.advance();
                            if next == '*' && self.peek() == '/' {
                                self.advance();
                                break;
                            }
                        }
                    } else {
                        self.add_token(TokenType::Divide)
                    }
                }
                '\n' => {
                    self.line += 1;
                }
                // Wabbit only allows single characters
                '\'' => {
                    // Two cases for rest of char:
                    //   c'
                    //   \n'
                    let one = self.advance();
                    let two = self.advance();
                    let three = self.peek();

                    if (one, two, three) == ('\\', 'n', '\'') {
                        self.advance();
                        self.add_literal_token(TokenType::Char, WabbitType::Char('\n'))
                    } else if two == '\'' {
                        self.add_literal_token(TokenType::Char, WabbitType::Char(one))
                    } else {
                        failed = true;
                        let err = err!(Msg::InvalidChar, self);
                        errors.push(err);
                    }
                }
                // whitespace
                ' ' | '\r' | '\t' => (),
                // numbers or identifiers
                _ => {
                    // check for an identifier first
                    if c.is_alphabetic() || c == '_' {
                        self.identifier()
                    } else if c.is_ascii_digit() || c == '.' {
                        if let Err(e) = self.number(c == '.') {
                            errors.push(e);
                            failed = true;
                        }
                    } else {
                        failed = true;
                        let err = err!(Msg::UnexpectedChar, self, c);
                        errors.push(err);
                    }
                }
            }
        }

        if failed {
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// scan all tokens

    pub fn scan(&mut self) -> Results<()> {
        let mut errors = Vec::new();
        let mut failed = false;
        while !self.is_end() {
            self.start = self.current;
            if let Err(errs) = self.scan_token() {
                errors.extend(errs);
                failed = true;
            }
        }
        if failed {
            Err(errors)
        } else {
            Ok(())
        }
    }
}
