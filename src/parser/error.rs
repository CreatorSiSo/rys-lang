use std::fmt::Display;

use crate::token::*;

#[derive(Debug)]
pub enum ParseError {
	TokenMismatch(Token, String),
	InvalidAssignmentTarget(Token),
}

impl ParseError {
	pub(super) fn token_mismatch<T>(token: &Token, msg: &str) -> Result<T, Self> {
		Err(ParseError::TokenMismatch(token.clone(), msg.into()))
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
			ParseError::InvalidAssignmentTarget(token) => {
				write!(
					f,
					"Line {}: Invalid assignment target {}",
					token.line, token.lexeme
				)
			}
		}
	}
}
