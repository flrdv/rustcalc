use std::collections::HashMap;
use crate::lex::error::Error;
use crate::parse::ast::{Binary, Node, Unary};
use crate::parse::ast::Node::{UnOp, BinOp};

pub struct Evaluator {
    vars: HashMap<String, f64>,
    fns: HashMap<String, Box<dyn Fn() -> f64>>
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            fns: HashMap::new()
        }
    }

    pub fn names(mut self, vars: HashMap<String, f64>) -> Self {
        self.vars = vars;
        self
    }

    pub fn functions(mut self, fns: HashMap<String, Box<dyn Fn() -> f64>>) -> Self {
        self.fns = fns;
        self
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
            Node::Call(name) => {
                if let Some(fun) = self.fns.get(name) {
                    return Ok(fun())
                }

                Err(Error::new("function not found".to_string()))
            },
        }
    }
}
