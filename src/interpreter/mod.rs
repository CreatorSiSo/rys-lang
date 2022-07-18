use crate::expr::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::literal::Literal::{self, *};

mod env;
mod error;
use env::Env;
use error::RuntimeError;

pub struct Interpreter {
	env: Env,
}

impl Interpreter {
	pub fn new() -> Self {
		Self { env: Env::new() }
	}

	pub fn evaluate(&mut self, ast: Vec<Stmt>) -> Result<(), RuntimeError> {
		for statement in ast {
			self.statement(statement)?;
		}
		Ok(())
	}

	pub fn statement(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
		match stmt {
			Stmt::Var(name, initializer) => {
				self.env.declare(name, self.expr(initializer)?);
			}
			Stmt::Expr(expr) => {
				self.expr(expr)?;
			}
			Stmt::Print(expr) => match self.expr(expr)? {
				Number(n) => println!("{n}"),
				String(s) => println!("\"{s}\""),
				True => println!("true"),
				False => println!("false"),
			},
		}
		Ok(())
	}

	pub fn expr(&self, expr: Expr) -> Result<Literal, RuntimeError> {
		Ok(match expr {
			Expr::Var(name) => self.env.get(&name).cloned()?,
			Expr::Literal(literal) => literal,
			Expr::Group(expr) => self.expr(*expr)?,
			Expr::Unary(op, expr) => self.unary(op, expr)?,
			Expr::Binary(expr_l, op, expr_r) => self.binary(expr_l, op, expr_r)?,
		})
	}

	fn unary(&self, op: UnaryOp, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
		let right = self.expr(*expr)?;

		match (op, right) {
			(UnaryOp::Not, True) => Ok(False),
			(UnaryOp::Not, False) => Ok(True),
			(UnaryOp::Neg, Number(n)) => Ok(Number(-n)),
			(op, literal) => RuntimeError::unary(op, literal),
		}
	}

	fn binary(
		&self,
		expr_l: Box<Expr>,
		op: BinaryOp,
		expr_r: Box<Expr>,
	) -> Result<Literal, RuntimeError> {
		let left = self.expr(*expr_l)?;
		let right = self.expr(*expr_r)?;

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
