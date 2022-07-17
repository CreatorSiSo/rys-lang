use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
	// Single-character tokens.
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Slash,
	Star,
	Semicolon,
	NewLine,

	// One or two character tokens.
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals.
	Identifier,
	String,
	Number,

	// Keywords.
	And,
	Struct,
	Else,
	False,
	Fun,
	For,
	If,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Let,
	While,

	Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
	pub typ: TokenType,
	pub lexeme: String,
	pub literal: Option<Literal>,
	pub line: usize,
	// col: u64,
}

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
