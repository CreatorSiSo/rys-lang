use crate::expr::Expr;
use crate::token::{Token, TokenType};

mod error;
use error::*;

pub struct Parser {
	tokens: Vec<Token>,
	errors: Vec<ParseError>,
	current: usize,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			tokens: Vec::new(),
			errors: Vec::new(),
			current: 0,
		}
	}

	pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Expr>, &[ParseError]> {
		self.tokens = tokens;
		self.errors.clear();
		self.current = 0;

		let statements = {
			let mut vec = Vec::new();
			loop {
				if self.is_at_end() {
					break vec;
				}
				match self.statement() {
					Ok(maybe_stmt) => {
						if let Some(stmt) = maybe_stmt {
							vec.push(stmt)
						}
					}
					Err(err) => self.errors.push(err),
				}
			}
		};

		if self.errors.is_empty() {
			Ok(statements)
		} else {
			Err(&self.errors)
		}
	}
}

/// Grammar definition
impl Parser {
	/// statement => expression ("\n" | ";" | EOF)
	fn statement(&mut self) -> Result<Option<Expr>, ParseError> {
		while self.matches(TokenType::NewLine) {
			self.advance();
		}

		let expr = self.expression()?;
		let peek = self.peek();
		match peek.typ {
			TokenType::NewLine | TokenType::Semicolon | TokenType::Eof => Ok(Some(expr)),
			_ => ParseError::token_mismatch(peek, "Expected newline or `;`"),
		}
	}

	/// expression => equality
	fn expression(&mut self) -> Result<Expr, ParseError> {
		self.equality()
	}

	/// equality => comparison (( "!=" | "==" ) comparison)*
	fn equality(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.comparison()?;

		while self.matches_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let operator = self.previous().clone();
			let right = Box::new(self.comparison()?);
			expr = Expr::Binary(Box::new(expr), operator, right);
		}

		Ok(expr)
	}

	/// comparison => term ((">" | ">=" | "<" | "<=") term)*
	fn comparison(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.term()?;

		while self.matches_any(&[
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		]) {
			let operator = self.previous().clone();
			let right = Box::new(self.term()?);
			expr = Expr::Binary(Box::new(expr), operator, right)
		}

		Ok(expr)
	}

	/// term => factor (("+" | "-") factor)*
	fn term(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let operator = self.previous().clone();
			let right = Box::new(self.factor()?);
			expr = Expr::Binary(Box::new(expr), operator, right);
		}

		Ok(expr)
	}

	/// factor => unary (("/" | "*") unary)*
	fn factor(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.unary()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let operator = self.previous().clone();
			let right = Box::new(self.unary()?);
			expr = Expr::Binary(Box::new(expr), operator, right);
		}

		Ok(expr)
	}

	/// unary => ("!" | "-") unary
	fn unary(&mut self) -> Result<Expr, ParseError> {
		if self.matches_any(&[TokenType::Bang, TokenType::Minus]) {
			let operator = self.previous().clone();
			let right = Box::new(self.unary()?);
			return Ok(Expr::Unary(operator, right));
		}

		self.primary()
	}

	/// primary => "(" expression ")", NUMBER | STRING | "true" | "false"
	fn primary(&mut self) -> Result<Expr, ParseError> {
		if self.matches_any(&[
			TokenType::True,
			TokenType::False,
			TokenType::Number,
			TokenType::String,
		]) {
			return Ok(Expr::Literal(
				self
					.previous()
					.literal
					.clone()
					.expect("Literal has no value!"),
			));
		}

		if self.matches(TokenType::LeftParen) {
			let expr = Box::new(self.expression()?);
			self.consume(TokenType::RightParen, "Expected closing `)`")?;
			return Ok(Expr::Group(expr));
		}

		ParseError::token_mismatch(
			self.advance(),
			"Expected expression, number, string, `true` or `false`",
		)
	}
}

/// Utility methods
impl Parser {
	fn consume(&mut self, until: TokenType, error_msg: &str) -> Result<&Token, ParseError> {
		if self.check(until) {
			return Ok(self.advance());
		}

		ParseError::token_mismatch(self.peek(), error_msg)
	}

	fn matches_any(&mut self, types: &[TokenType]) -> bool {
		for typ in types {
			if self.matches(*typ) {
				return true;
			}
		}

		false
	}

	fn matches(&mut self, typ: TokenType) -> bool {
		if self.check(typ) {
			self.advance();
			return true;
		}

		false
	}

	fn check(&self, typ: TokenType) -> bool {
		if self.is_at_end() {
			return false;
		}
		self.peek().typ == typ
	}

	fn advance(&mut self) -> &Token {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.previous()
	}

	fn is_at_end(&self) -> bool {
		self.peek().typ == TokenType::Eof
	}

	fn previous(&self) -> &Token {
		&self.tokens[self.current - 1]
	}

	fn peek(&self) -> &Token {
		&self.tokens[self.current]
	}
}
