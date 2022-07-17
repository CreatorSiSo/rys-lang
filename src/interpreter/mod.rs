use crate::expr::{
	Expr,
	UnaryOp::{self, *},
};
use crate::literal::Literal::{self, *};
use crate::token::Token;

mod error;
use error::RuntimeError;

pub struct Interpreter {}

impl Interpreter {
	pub fn evaluate(expr: Expr) -> Result<Literal, RuntimeError> {
		Ok(match expr {
			Expr::Literal(literal) => literal,
			Expr::Group(expr) => Self::evaluate(*expr)?,
			Expr::Unary(op, expr) => Self::unary(op, expr)?,
			Expr::Binary(expr_l, op, expr_r) => Self::binary(expr_l, op, expr_r)?,
		})
	}

	fn unary(op: UnaryOp, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
		let right = Self::evaluate(*expr)?;

		match (op, right) {
			(Not, True) => Ok(False),
			(Not, False) => Ok(True),
			(Neg, Number(n)) => Ok(Number(-n)),

			(Neg, True | False) => {
				RuntimeError::forbidden_type("Cannot apply unary operator `-` to `bool`")
			}
			(Not, Number(_)) => {
				RuntimeError::forbidden_type("Cannot apply unary operator `!` to `number`")
			}
			(Neg, String(_)) => {
				RuntimeError::forbidden_type("Cannot apply unary operator `-` to `string`")
			}
			(Not, String(_)) => {
				RuntimeError::forbidden_type("Cannot apply unary operator `!` to `string`")
			}
		}
	}

	fn binary(expr_l: Box<Expr>, op: Token, expr_r: Box<Expr>) -> Result<Literal, RuntimeError> {
		let left = Self::evaluate(*expr_l)?;
		let right = Self::evaluate(*expr_r)?;

		Ok(Literal::False)
	}
}
