use crate::expr::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::literal::Literal::{self, *};

mod env;
mod error;
use env::Env;
use error::RuntimeError;

pub struct Interpreter<'a> {
	env: Env<'a>,
}

impl Interpreter<'_> {
	pub fn new() -> Self {
		Self { env: Env::new() }
	}

	pub fn evaluate(&mut self, ast: Vec<Stmt>) -> Result<(), RuntimeError> {
		for statement in ast {
			self.statement(statement)?;
		}
		Ok(())
	}

	fn statement(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
		match stmt {
			Stmt::Var {
				name,
				initializer,
				mutable,
			} => {
				let value = self.expr(initializer)?;
				self.env.declare(name, value, mutable);
			}
			Stmt::Expr(expr) => {
				self.expr(expr)?;
			}
			Stmt::Print(expr) => match self.expr(expr)? {
				Number(n) => println!("{n}"),
				String(s) => println!("{s}"),
				True => println!("true"),
				False => println!("false"),
			},
		}
		Ok(())
	}

	fn expr(&mut self, expr: Expr) -> Result<Literal, RuntimeError> {
		Ok(match expr {
			Expr::Var(name) => self.env.get(&name).cloned()?,
			Expr::Assign(name, expr) => {
				let value = self.expr(*expr)?;
				self.env.set(&name, value.clone())?;
				value
			}
			Expr::Literal(literal) => literal,
			Expr::Group(expr) => self.expr(*expr)?,
			Expr::Unary(op, expr) => self.unary(op, expr)?,
			Expr::Binary(expr_l, op, expr_r) => self.binary(expr_l, op, expr_r)?,
		})
	}

	fn unary(&mut self, op: UnaryOp, expr: Box<Expr>) -> Result<Literal, RuntimeError> {
		let right = self.expr(*expr)?;

		match (op, right) {
			(UnaryOp::Not, True) => Ok(False),
			(UnaryOp::Not, False) => Ok(True),
			(UnaryOp::Neg, Number(n)) => Ok(Number(-n)),
			(op, literal) => RuntimeError::unary(op, literal),
		}
	}

	fn binary(
		&mut self,
		expr_l: Box<Expr>,
		op: BinaryOp,
		expr_r: Box<Expr>,
	) -> Result<Literal, RuntimeError> {
		let left = self.expr(*expr_l)?;
		let right = self.expr(*expr_r)?;

		// TODO: Clean this up evme more!
		match op {
			BinaryOp::Equal => Ok(if left == right { True } else { False }),
			BinaryOp::NotEqual => Ok(if left != right { True } else { False }),
			BinaryOp::Greater => Self::comparison(left, right, |l, r| l > r),
			BinaryOp::GreaterEqual => Self::comparison(left, right, |l, r| l >= r),
			BinaryOp::Less => Self::comparison(left, right, |l, r| l < r),
			BinaryOp::LessEqual => Self::comparison(left, right, |l, r| l <= r),
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
			BinaryOp::Sub => Self::algebraic(left, right, |l, r| l - r, RuntimeError::substraction),
			BinaryOp::Mul => Self::algebraic(left, right, |l, r| l - r, RuntimeError::multiplication),
			BinaryOp::Div if right == Number(0f64) => Err(RuntimeError::DivideByZero),
			BinaryOp::Div => Self::algebraic(left, right, |l, r| l - r, RuntimeError::division),
		}
	}

	fn comparison<F>(left: Literal, right: Literal, cmp_fn: F) -> Result<Literal, RuntimeError>
	where
		F: Fn(f64, f64) -> bool,
	{
		if let Number(l) = left {
			if let Number(r) = right {
				return Ok(if cmp_fn(l, r) { True } else { False });
			}
		}

		RuntimeError::comparison(left, right)
	}

	fn algebraic<F, E>(
		left: Literal,
		right: Literal,
		algebra_fn: F,
		err: E,
	) -> Result<Literal, RuntimeError>
	where
		F: Fn(f64, f64) -> f64,
		E: Fn(Literal, Literal) -> Result<Literal, RuntimeError>,
	{
		if let Number(l) = left {
			if let Number(r) = right {
				return Ok(Number(algebra_fn(l, r)));
			}
		}

		err(left, right)
	}
}
