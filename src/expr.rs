use crate::literal::Literal;
use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
	Binary(Box<Expr>, Token /* BinaryOp */, Box<Expr>),
	Unary(UnaryOp, Box<Expr>),
	Literal(Literal),
	Group(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOp {
	Neg,
	Not,
}
