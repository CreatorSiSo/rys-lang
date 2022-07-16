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
