use std::collections::HashMap;
use crate::lex::error::Error;
use crate::parse::ast::{Binary, Node, Unary};
use crate::parse::ast::Node::{UnOp, BinOp};

pub struct Evaluator {
    vars: HashMap<String, f64>
}

impl Evaluator {
    pub fn new(vars: HashMap<String, f64>) -> Self {
        Self { vars }
    }

    pub fn evaluate(&self, node: &Node) -> Result<f64, Error> {
        match node {
            UnOp(Unary::Pos, right) => Ok(self.evaluate(right)?),
            UnOp(Unary::Neg, right) => Ok(-self.evaluate(right)?),
            BinOp(Binary::Add, left, right) => Ok(self.evaluate(left)? + self.evaluate(right)?),
            BinOp(Binary::Sub, left, right) => Ok(self.evaluate(left)? - self.evaluate(right)?),
            BinOp(Binary::Mul, left, right) => Ok(self.evaluate(left)? * self.evaluate(right)?),
            BinOp(Binary::Div, left, right) => Ok(self.evaluate(left)? / self.evaluate(right)?),
            BinOp(Binary::Pow, left, right) => Ok(self.evaluate(left)?.powf(self.evaluate(right)?)),

            Node::Const(num) => Ok(*num),
            Node::Id(name) => {
                if let Some(value) = self.vars.get(name) {
                    return Ok(*value)
                }

                Err(Error::new("name not found".to_string()))
            },
            Node::Call(_) => Ok(-1f64),
        }
    }
}
