use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
	Binary(Box<Expr>, Token /* BinaryOp */, Box<Expr>),
	Unary(Token /* UnaryOp */, Box<Expr>),
	Literal(Literal),
	Group(Box<Expr>),
}

// pub enum UnaryOp {
// 	Negate,
// 	Not,
// }
