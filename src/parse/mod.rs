mod ast;
mod stream;

use std::ops::Deref;
use crate::lex;
use crate::parse::stream::Stream;

pub struct Parser {
    stream: Stream<lex::Token>
}

impl<'a> Parser {
    pub fn new(stream: Stream<lex::Token>) -> Self {
        Self { stream }
    }

    pub fn parse(&mut self) -> Vec<ast::Node> {
        let mut stmts: Vec<ast::Node> = Vec::with_capacity(self.stream.len());

        while !self.stream.empty() {
            stmts.push(self.stmt())
        }

        stmts
    }

    fn stmt(&mut self) -> ast::Node {
        let term = self.term();

        ast::Node::Id("hui".to_string())
        // addition and subtraction
    }

    fn term(&mut self) -> ast::Node {
        // multiplication and division
        ast::Node::Id("hui".to_string())
    }

    fn exp(&mut self) -> ast::Node {
        let factor = self.factor();

        // match self.stream.preview() {
        //     lex::Token::Operator(lex::Op::Pow) =>
        // }

        ast::Node::Id("hui".to_string())
    }

    fn factor(&mut self) -> ast::Node {
        match self.stream.pop().unwrap() {
            lex::Token::Const(literal) => {
                ast::Node::Const(literal.parse::<i64>().unwrap())
            },
            lex::Token::ID(literal) => {
                ast::Node::Id(literal.clone())
            },
            // lex::Token::LParen =>
            _ => panic!("unrecognized lexeme")
        }
        // const, id and parenthesis
    }
}
