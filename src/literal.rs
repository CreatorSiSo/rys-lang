use std::fmt::Display;

#[derive(Debug, Clone)]
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
