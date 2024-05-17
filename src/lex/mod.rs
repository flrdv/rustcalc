pub mod error;
mod radix;

use error::Error;
use crate::lex::Token::{Comma, Const, LParen, Operator, RParen, Unknown};

pub struct Lexer<'a> {
    string: &'a str,
    optree: radix::Node<Op>
}

impl<'a> Lexer<'a> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            string,
            optree: {
                let mut radix = radix::Node::new();
                radix.insert("+", Op::Add);
                radix.insert("-", Op::Sub);
                radix.insert("*", Op::Mul);
                radix.insert("/", Op::Div);
                radix.insert("**", Op::Pow);
                radix
            }
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut lexemes: Vec<Token> = Vec::new();

        while self.string.len() > 0 {
            lexemes.push(self.lexeme()?);
        }

        Ok(lexemes)
    }

    fn lexeme(&mut self) -> Result<Token, Error> {
        self.whitespace();

        match self.string.chars().nth(0).unwrap() {
            '0'..='9' => self.int(),
            'a'..='z' | 'A'..='Z' | '_' => self.id(),
            '+' | '-' | '*' | '/' => self.op(),
            '(' => Ok(self.lparen()),
            ')' => Ok(self.rparen()),
            ',' => Ok(self.comma()),
            _ => Ok(self.unknown())
        }
    }

    fn whitespace(&mut self) {
        self.advance(self.end_of_whitespace());
    }

    fn end_of_whitespace(&self) -> usize {
        for (i, ch) in self.string.chars().enumerate() {
            match ch {
                ' ' | '\n' | '\t' => (),
                _ => {
                    return i
                }
            }
        }

        self.string.len()
    }

    fn comma(&mut self) -> Token {
        self.advance(1);
        Comma
    }

    fn lparen(&mut self) -> Token {
        self.advance(1);
        LParen
    }

    fn rparen(&mut self) -> Token {
        self.advance(1);
        RParen
    }

    fn int(&mut self) -> Result<Token, Error> {
        for (i, ch) in self.string.chars().enumerate() {
            match ch {
                '0'..='9' | '.' => (),
                _ => return Ok(Const(self.advance(i)))
            }
        }

        Ok(Const(self.advance(self.string.len())))
    }

    fn id(&mut self) -> Result<Token, Error> {
        for (i, ch) in self.string.chars().enumerate() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => (),
                _ => {
                    return Ok(Token::ID(self.advance(i)))
                }
            }
        }

        Ok(Token::ID(self.advance(self.string.len()).to_string()))
    }

    fn op(&mut self) -> Result<Token, Error> {
        match self.find_op_end() {
            None => Err(Error::new("encountered an incomplete operator".to_string())),
            Some((index, op)) => {
                self.advance(index);
                Ok(Operator(op))
            }
        }
    }

    fn find_op_end(&self) -> Option<(usize, Op)> {
        let mut cursor = self.optree.cursor();

        for (i, ch) in self.string.chars().enumerate() {
            if !cursor.visit(ch) {
                return match cursor.payload() {
                    None => None,
                    Some(op) => Some((i, op))
                };
            }
        }

        match cursor.payload() {
            None => None,
            Some(op) => Some((self.string.len(), op))
        }
    }

    fn unknown(&mut self) -> Token {
        match self.string.find('\n') {
            None => Unknown(self.advance(self.string.len())),
            Some(pos) => Unknown(self.advance(pos))
        }
    }

    fn advance(&mut self, n: usize) -> String {
        let prefix = &self.string[..n];
        self.string = &self.string[n..];
        prefix.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown(String),
    Operator(Op),
    Const(String),
    ID(String),
    LParen,
    RParen,
    Comma
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
