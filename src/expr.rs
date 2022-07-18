use crate::literal::Literal;

#[derive(Debug)]
pub enum Expr {
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Unary(UnaryOp, Box<Expr>),
	Literal(Literal),
	Group(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOp {
	Neg,
	Not,
}

// TODO: Maybe add power
#[derive(Debug)]
pub enum BinaryOp {
	Equal,
	NotEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	Add,
	Substract,
	Multiply,
	Divide,
}
