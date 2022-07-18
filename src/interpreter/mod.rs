use crate::expr::{BinaryOp, Expr, UnaryOp};
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
			(UnaryOp::Not, True) => Ok(False),
			(UnaryOp::Not, False) => Ok(True),
			(UnaryOp::Neg, Number(n)) => Ok(Number(-n)),
			(op, literal) => RuntimeError::unary(op, literal),
		}
	}

	fn binary(expr_l: Box<Expr>, op: BinaryOp, expr_r: Box<Expr>) -> Result<Literal, RuntimeError> {
		let left = Self::evaluate(*expr_l)?;
		let right = Self::evaluate(*expr_r)?;

		// TODO: Clean this up!
		match op {
			BinaryOp::Equal => Ok(if left == right { True } else { False }),
			BinaryOp::NotEqual => Ok(if left != right { True } else { False }),
			BinaryOp::Greater => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l > r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			BinaryOp::GreaterEqual => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l >= r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			BinaryOp::Less => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l < r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			BinaryOp::LessEqual => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(if l <= r { True } else { False });
					}
				}

				RuntimeError::comparison(left, right)
			}
			BinaryOp::Add => match (&left, &right) {
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
			BinaryOp::Substract => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(Number(l - r));
					}
				}

				RuntimeError::substraction(left, right)
			}
			BinaryOp::Multiply => {
				if let Number(l) = left {
					if let Number(r) = right {
						return Ok(Number(l * r));
					}
				}

				RuntimeError::multiplication(left, right)
			}
			BinaryOp::Divide => {
				if let Number(l) = left {
					if let Number(r) = right {
						return if r == 0f64 {
							Err(RuntimeError::DivideByZero)
						} else {
							Ok(Number(l / r))
						};
					}
				}

				RuntimeError::division(left, right)
			}
		}
	}
}
