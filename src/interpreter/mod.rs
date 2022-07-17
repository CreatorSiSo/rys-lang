use crate::literal::Literal;
use crate::parser::Expr;
use crate::token::{Token, TokenType};

mod error;
use error::RuntimeError;

pub struct Interpreter {}

impl Interpreter {
	pub fn new() -> Self {
		Self {}
	}

	pub fn evaluate(expr: Expr) -> Result<Literal, RuntimeError> {
		Ok(match expr {
			Expr::Literal(literal) => literal,
			Expr::Group(expr) => Self::evaluate(*expr)?,
			Expr::Unary(op, expr) => Self::unary(op, expr)?,
			Expr::Binary(expr_l, op, expr_r) => Self::binary(expr_l, op, expr_r)?,
		})
	}

	fn unary(op: Token, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
		let right = Self::evaluate(*expr)?;

		match right {
			Literal::String(_) => Err(RuntimeError::ForbiddenType),
			mut literal => {
				match (op.typ, &literal) {
					(TokenType::Bang, Literal::True) => literal = Literal::False,
					(TokenType::Bang, Literal::False) => literal = Literal::True,
					(TokenType::Bang, Literal::Number(_)) => return Err(RuntimeError::ForbiddenType),

					(TokenType::Minus, Literal::Number(n)) => literal = Literal::Number(-n),
					(TokenType::Minus, Literal::True | Literal::False) => {
						return Err(RuntimeError::ForbiddenType)
					}
					_ => { /* Unreachable TODO: Remove */ }
				}

				// TODO: Remove
				return Ok(literal);
			}
		}
	}

	fn binary(expr_l: Box<Expr>, op: Token, expr_r: Box<Expr>) -> Result<Literal, RuntimeError> {
		let left = Self::evaluate(*expr_l)?;
		let right = Self::evaluate(*expr_r)?;

		Ok(Literal::False)
	}
}
