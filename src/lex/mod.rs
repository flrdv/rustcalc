mod error;
mod radix;

use error::Error;
use crate::lex::Token::{Operator, Unknown};

pub struct Lexer<'a> {
    string: &'a str,
    optree: radix::Node<Op>
}

impl Lexer<'_> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            string,
            optree: {
                let mut radix = radix::Node::new();
                radix.insert("+", &Op::Add);
                radix.insert("-", &Op::Sub);
                radix.insert("*", &Op::Mul);
                radix.insert("/", &Op::Div);
                radix.insert("**", &Op::Pow);
                radix
            }
        }
    }

    pub fn lex<'a, 'b>(&'b mut self) -> Result<Vec<Token<'a>>, Error> {
        let mut lexemes: Vec<Token<'a>> = Vec::new();

        while self.string.len() > 0 {
            lexemes.push(self.lexeme()?);
        }

        Ok(lexemes)
    }

    fn lexeme<'a>(&mut self) -> Result<Token<'a>, Error> {
        match self.string.chars().nth(0).unwrap() {
            '0'..='9' => self.int(),
            'a'..='z' | 'A'..='Z' | '_' => self.id(),
            '+' | '-' | '*' | '/' => self.op(),
            _ => Ok(self.unknown())
        }
    }

    fn int<'a>(&mut self) -> Result<Token<'a>, Error> {
        for (i, ch) in self.string.chars().enumerate() {
           if !(ch >= '0' && ch <= '9') {
               return Ok(Token::Const(self.advance(i)))
           }
        }

        Ok(Token::Const(self.advance(self.string.len())))
    }

    fn id<'a>(&mut self) -> Result<Token<'a>, Error> {
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

    fn op<'a>(&mut self) -> Result<Token<'a>, Error> {
        let mut cursor = self.optree.cursor();

        for (i, ch) in self.string.chars().enumerate().skip(1) {
            if !cursor.visit(ch) {
                self.advance(i);
                if let Some(op) = cursor.payload() {
                    return Ok(Operator(op))
                }

                return Err(Error::new("invalid operator".to_string()))
            }
        }

        Err(Error::new("encountered an incomplete operator".to_string()))
    }

    fn unknown<'a>(&mut self) -> Token<'a> {
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

#[derive(Debug)]
pub enum Token<'a> {
    Unknown(String),
    Operator(&'a Op),
    Const(String),
    ID(String)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
