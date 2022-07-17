use crate::expr::{
	BinaryOp::{self, *},
	Expr,
	UnaryOp::{self, *},
};
use crate::literal::Literal::{self, *};

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

	fn binary(expr_l: Box<Expr>, op: BinaryOp, expr_r: Box<Expr>) -> Result<Literal, RuntimeError> {
		let left = Self::evaluate(*expr_l)?;
		let right = Self::evaluate(*expr_r)?;

		match op {
			Equal => Ok(if left == right { True } else { False }),
			NotEqual => Ok(if left != right { True } else { False }),
			Greater => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l > r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			GreaterEqual => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l >= r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			Less => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l < r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			LessEqual => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l <= r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			Plus => match (&left, &right) {
				(Number(l), Number(r)) => return Ok(Number(l + r)),
				(Number(l), String(r)) => {
					let mut value = l.to_string();
					value.push_str(&r);
					Ok(String(value))
				}
				(String(l), r) => {
					let mut value = l.clone();

					if let String(ref inner) = r {
						value.push_str(inner);
					}
					if let Number(inner) = r {
						value.push_str(&inner.to_string());
					}
					if &True == r {
						value.push_str("true");
					}
					if &False == r {
						value.push_str("false");
					}

					Ok(String(value))
				}
				(True, String(inner)) => {
					let mut value = "true".to_string();
					value.push_str(&inner);
					Ok(String(value))
				}
				(False, String(inner)) => {
					let mut value = "false".to_string();
					value.push_str(&inner);
					Ok(String(value))
				}
				_ => RuntimeError::addition(left, right),
			},
			Minus => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(Number(l - r));
					}
				}

				RuntimeError::subtraction(left, right)
			}
		}
	}
}
