use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Number(f64),
	String(String),
	True,
	False,
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::True => write!(f, "true"),
			Literal::False => write!(f, "false"),
			Literal::String(value) => write!(f, "{value}"),
			Literal::Number(value) => write!(f, "{value}"),
		}
	}
}

impl Literal {
	pub fn to_type_string(&self) -> String {
		match self {
			Literal::Number(_) => "number".into(),
			Literal::String(_) => "string".into(),
			Literal::True => "true".into(),
			Literal::False => "false".into(),
		}
	}
}
