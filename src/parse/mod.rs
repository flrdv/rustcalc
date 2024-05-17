pub mod ast;
pub mod stream;

use crate::lex::{Op, Token};
use crate::lex::Token::RParen;
use crate::parse::ast::Node::{BinOp, UnOp};
use crate::parse::ast::{Binary, Node, Unary};
use crate::parse::stream::Stream;

pub struct Parser {
    stream: Stream<Token>
}

impl<'a> Parser {
    pub fn new(stream: Stream<Token>) -> Self {
        Self { stream }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut stmts: Vec<Node> = Vec::with_capacity(self.stream.len());

        while !self.stream.empty() {
            stmts.push(self.stmt())
        }

        stmts
    }

    fn stmt(&mut self) -> Node {
        let mut expr = self.expr();

        loop {
            match self.stream.pop() {
                Some(Token::Operator(Op::Add)) => {
                    expr = BinOp(Binary::Add, Box::new(expr), Box::new(self.expr()))
                }
                Some(Token::Operator(Op::Sub)) => {
                    expr = BinOp(Binary::Sub, Box::new(expr), Box::new(self.expr()))
                }
                None => {
                    return expr
                }
                _ => {
                    self.stream.back();
                    return expr
                }
            }
        }
    }

    fn expr(&mut self) -> Node {
        let mut exp = self.exp();

        loop {
            match self.stream.pop() {
                Some(Token::Operator(Op::Mul)) => {
                    exp = BinOp(Binary::Mul, Box::new(exp), Box::new(self.exp()))
                },
                Some(Token::Operator(Op::Div)) => {
                    exp = BinOp(Binary::Div, Box::new(exp), Box::new(self.exp()))
                },
                None => {
                    return exp
                },
                _ => {
                    self.stream.back();
                    return exp
                }
            }
        }
    }

    fn exp(&mut self) -> Node {
        let term = self.term();

        match self.stream.pop() {
            Some(Token::Operator(Op::Pow)) => {
                BinOp(Binary::Pow, Box::new(term), Box::new(self.exp()))
            },
            None => term,
            _ => {
                self.stream.back();
                term
            }
        }
    }
    
    fn term(&mut self) -> Node {
        let factor = self.factor();
        if let Node::Id(name) = &factor {
            match self.stream.preview() {
                Some(Token::LParen) => {
                    // parse arguments here
                    self.stream.pop(); // lparen
                    self.stream.pop(); // rparen; fixme: check if it's closing parenthesis

                    return Node::Call(name.clone())
                },
                _ => ()
            }
        }

        factor
    }

    fn factor(&mut self) -> Node {
        let token = self.stream.pop().unwrap();
        match token {
            Token::Const(literal) => {
                Node::Const(literal.parse::<f64>().unwrap())
            },
            Token::ID(literal) => {
                Node::Id(literal.clone())
            },
            Token::LParen => {
                let stmt = self.stmt();
                match self.stream.pop().unwrap() {
                    RParen => stmt,
                    _ => panic!("expected closing parenthesis")
                }
            },
            Token::Operator(Op::Add) => {
                UnOp(Unary::Pos, Box::new(self.exp()))
            },
            Token::Operator(Op::Sub) => {
                UnOp(Unary::Neg, Box::new(self.exp()))
            },
            _ => {
                panic!("unexpected lexeme: {:?}", token)
            }
        }
    }
}
