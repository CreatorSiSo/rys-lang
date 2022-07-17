use crate::literal::Literal;
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

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.typ)?;
		if let TokenType::Identifier = self.typ {
			write!(f, "({})", self.lexeme)?
		}
		if let Some(value) = &self.literal {
			write!(f, "({})", value)?
		}
		Ok(())
	}
}
