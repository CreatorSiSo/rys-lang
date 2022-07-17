use crate::literal::Literal;

#[derive(Debug)]
pub enum RuntimeError {
	ForbiddenType(String),
	TypeMismatch(String),
}

impl RuntimeError {
	pub fn forbidden_type<T>(msg: &str) -> Result<T, Self> {
		Err(Self::ForbiddenType(msg.into()))
	}

	pub fn comparison<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot compare `{}` with `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub fn addition<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot add `{}` to `{}`",
			left.to_type_string(),
			right.to_type_string(),
		)))
	}

	pub fn subtraction<T>(left: Literal, right: Literal) -> Result<T, Self> {
		Err(Self::TypeMismatch(format!(
			"Cannot subtract `{}` from `{}`",
			right.to_type_string(),
			left.to_type_string(),
		)))
	}
}
