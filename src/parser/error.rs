use std::fmt::Display;

use crate::token::*;

#[derive(Debug)]
pub enum ParseError {
	TokenMismatch(Token, String),
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: &Token, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch(token.to_owned(), msg.into()))
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::TokenMismatch(token, msg) => {
				write!(
					f,
					"Line {}: {msg} got `{}`.",
					token.line,
					token.lexeme.escape_debug()
				)
			}
		}
	}
}
