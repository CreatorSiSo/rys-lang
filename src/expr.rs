use crate::token::*;

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

#[derive(Debug)]
pub enum Literal {
	Number(f64),
	String(String),
	True,
	False,
}
