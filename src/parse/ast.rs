#[derive(Debug)]
pub enum Node {
    UnOp(Unary, Box<Node>),
    BinOp(Binary, Box<Node>, Box<Node>),
    Const(i64),
    Id(String),
    Call(String)
}

#[derive(Debug)]
pub enum Binary {
    Add,
    Sub,
    Div,
    Mul,
    Pow
}

#[derive(Debug)]
pub enum Unary {
    Pos,
    Neg
}
