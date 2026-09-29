use crate::Spanned;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Spanned<Literal>),
    BinaryOp {
        lhs: Spanned<Box<Expr>>,
        op: Spanned<Operator>,
        rhs: Spanned<Box<Expr>>,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Semicolon,
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
}
