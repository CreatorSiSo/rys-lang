use crate::lexer;

#[derive(Debug)]
pub enum Error {}

pub struct Parser {
	tokens: Vec<lexer::Token>,
	errors: Vec<Error>,
}

impl Default for Parser {
	fn default() -> Self {
		Self {
			tokens: Vec::new(),
			errors: Vec::new(),
		}
	}
}

impl Parser {
	pub fn parse(&mut self, tokens: Vec<lexer::Token>) -> Result<(), &[Error]> {
		self.tokens = tokens;

		if self.errors.is_empty() {
			Ok(())
		} else {
			Err(&self.errors)
		}
	}
}
