use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	True,
	False,
	Number(f64),
	String(String),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::True => write!(f, "true"),
			Literal::False => write!(f, "false"),
			Literal::Number(value) => write!(f, "{value}"),
			Literal::String(value) => write!(f, "{value}"),
		}
	}
}

impl Literal {
	pub fn to_type_string(&self) -> String {
		match self {
			Literal::True => "true".into(),
			Literal::False => "false".into(),
			Literal::Number(_) => "number".into(),
			Literal::String(_) => "string".into(),
		}
	}
}
