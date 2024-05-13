pub enum Node {
    UnOp(Unary, Box<Node>),
    BinOp(Binary, Box<Node>, Box<Node>),
    Const(i64),
    Id(String),
}

pub enum Binary {
    Add,
    Sub,
    Div,
    Mul,
    Pow
}

pub enum Unary {
    Pos,
    Neg
}
