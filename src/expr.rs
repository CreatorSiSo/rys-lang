use crate::literal::Literal;

#[derive(Debug)]
pub enum Stmt {
	Expr(Expr),
	Print(Expr),
	Var {
		name: String,
		initializer: Expr,
		mutable: bool,
	},
}

#[derive(Debug)]
pub enum Expr {
	Binary(Box<Expr>, BinaryOp, Box<Expr>),
	Unary(UnaryOp, Box<Expr>),
	Literal(Literal),
	Group(Box<Expr>),
	// TODO: Should this really be an expression or should assignment be a statement?
	Assign(String, Box<Expr>),
	Var(String),
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
