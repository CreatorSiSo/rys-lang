use crate::literal::Literal;
use crate::token::Token;

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
