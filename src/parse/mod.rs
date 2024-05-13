pub mod ast;
pub mod stream;

use std::ops::Deref;
use crate::lex;
use crate::lex::{Op, Token};
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
            match self.stream.pop().unwrap() {
                Token::Operator(Op::Add) => {
                    expr = BinOp(Binary::Mul, Box::new(expr), Box::new(self.expr()))
                }
                Token::Operator(Op::Sub) => {
                    expr = BinOp(Binary::Div, Box::new(expr), Box::new(self.expr()))
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
            match self.stream.pop().unwrap() {
                Token::Operator(Op::Mul) => {
                    exp = BinOp(Binary::Mul, Box::new(exp), Box::new(self.exp()))
                }
                Token::Operator(Op::Div) => {
                    exp = BinOp(Binary::Div, Box::new(exp), Box::new(self.exp()))
                }
                _ => {
                    self.stream.back();
                    return exp
                }
            }
        }
    }

    fn exp(&mut self) -> Node {
        let term = self.term();

        match self.stream.pop().unwrap() {
            Token::Operator(Op::Pow) => {
                BinOp(Binary::Pow, Box::new(term), Box::new(self.exp()))
            },
            _ => {
                self.stream.back();
                term
            }
        }
    }
    
    fn term(&mut self) -> Node {
        let factor = self.factor();
        if let Node::Id(name) = &factor  {
            if let Token::LParen = self.stream.pop().unwrap() {
                return Node::Call(name.clone())
            }

            self.stream.back()
        }

        factor
    }

    fn factor(&mut self) -> Node {
        match self.stream.pop().unwrap() {
            Token::Const(literal) => {
                Node::Const(literal.parse::<i64>().unwrap())
            },
            Token::ID(literal) => {
                Node::Id(literal.clone())
            },
            Token::LParen => self.stmt(),
            Token::Operator(Op::Add) => {
                UnOp(Unary::Pos, Box::new(self.factor()))
            },
            Token::Operator(Op::Sub) => {
                UnOp(Unary::Neg, Box::new(self.factor()))
            },
            _ => panic!("unexpected lexeme")  // do we really need this? Why not just return what we've got?
        }
    }
}
