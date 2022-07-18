use crate::expr::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::token::{Token, TokenType};

mod error;
use error::ParseError;

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

	pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Stmt>, &[ParseError]> {
		self.tokens = tokens;
		self.errors.clear();
		self.current = 0;

		let statements = self.program();

		if self.errors.is_empty() {
			Ok(statements)
		} else {
			Err(&self.errors)
		}
	}
}

/// Grammar definition
impl Parser {
	// program => statement* EOF
	fn program(&mut self) -> Vec<Stmt> {
		let mut vec = Vec::new();
		loop {
			if self.is_at_end() {
				break vec;
			}
			while self.matches(TokenType::NewLine) {
				self.advance();
			}
			match self.declaration() {
				Ok(stmt) => vec.push(stmt),
				Err(err) => self.errors.push(err),
			}
		}
	}

	/// declaration => const_decl | mut_decl | statement
	fn declaration(&mut self) -> Result<Stmt, ParseError> {
		// "const" part of const_decl
		if self.matches(TokenType::Const) {
			return self.const_decl();
		}
		// "mut" part of mut_decl
		if self.matches(TokenType::Mut) {
			return self.mut_decl();
		}
		self.statement()
	}

	/// const_decl => "const" IDENTIFIER "=" expression ";"
	fn const_decl(&mut self) -> Result<Stmt, ParseError> {
		let name = self
			.consume(TokenType::Identifier, "Expected variable name")?
			.lexeme
			.clone();
		self.consume(TokenType::Equal, "Expected `=`")?;
		let initializer = self.expression()?;
		let next = self.advance();
		match next.typ {
			TokenType::Semicolon | TokenType::Eof => Ok(Stmt::Var {
				name,
				initializer,
				mutable: false,
			}),
			_ => ParseError::token_mismatch(next, "Expected `;`"),
		}
	}

	/// mut_decl => "mut" IDENTIFIER "=" expression ";"
	fn mut_decl(&mut self) -> Result<Stmt, ParseError> {
		let name = self
			.consume(TokenType::Identifier, "Expected variable name")?
			.lexeme
			.clone();
		self.consume(TokenType::Equal, "Expected `=`")?;
		let initializer = self.expression()?;
		let next = self.advance();
		match next.typ {
			TokenType::Semicolon | TokenType::Eof => Ok(Stmt::Var {
				name,
				initializer,
				mutable: true,
			}),
			_ => ParseError::token_mismatch(next, "Expected `;`"),
		}
	}

	/// statement => exprStmt | printStmt
	fn statement(&mut self) -> Result<Stmt, ParseError> {
		// print_stmt => "print" expression (";" | EOF)
		if self.matches(TokenType::Print) {
			return self.print_stmt();
		}
		// block "{" declaration* "}"
		if self.matches(TokenType::LeftBrace) {
			return Ok(Stmt::Block(self.block()?));
		}
		self.expr_stmt()
	}

	/// block "{" declaration* "}"
	fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
		let mut statements = Vec::new();
		loop {
			if self.check(TokenType::RightBrace) || self.is_at_end() {
				self.advance();
				break Ok(statements);
			}
			while self.matches(TokenType::NewLine) {
				self.advance();
			}
			match self.declaration() {
				Ok(stmt) => statements.push(stmt),
				Err(err) => self.errors.push(err),
			}
		}
	}

	fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
		let expr = self.expression()?;
		let next = self.advance();
		match next.typ {
			TokenType::Semicolon | TokenType::Eof => Ok(Stmt::Print(expr)),
			_ => ParseError::token_mismatch(next, "Expected `;`"),
		}
	}

	/// expr_stmt => expression (";" | EOF)
	fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
		let expr = self.expression()?;
		let next = self.advance();
		match next.typ {
			TokenType::Semicolon | TokenType::Eof => Ok(Stmt::Expr(expr)),
			_ => ParseError::token_mismatch(next, "Expected `;`"),
		}
	}

	/// expression => equality
	fn expression(&mut self) -> Result<Expr, ParseError> {
		self.assignment()
	}

	/// assignment => identifier "=" (assignment | equality)
	fn assignment(&mut self) -> Result<Expr, ParseError> {
		let expr = self.equality()?;

		if self.matches(TokenType::Equal) {
			let equals = self.previous().clone();
			let value = Box::new(self.assignment()?);

			if let Expr::Var(name) = expr {
				return Ok(Expr::Assign(name, value));
			}

			return Err(ParseError::InvalidAssignmentTarget(equals));
		}

		Ok(expr)
	}

	/// equality => comparison (( "!=" | "==" ) comparison)*
	fn equality(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.comparison()?;

		while self.matches_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let typ = self.previous().typ;
			let right = Box::new(self.comparison()?);

			expr = if typ == TokenType::BangEqual {
				Expr::Binary(Box::new(expr), BinaryOp::NotEqual, right)
			} else {
				Expr::Binary(Box::new(expr), BinaryOp::Equal, right)
			};
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
			let typ = self.previous().typ;
			let right = Box::new(self.term()?);

			expr = Expr::Binary(
				Box::new(expr),
				match typ {
					TokenType::Greater => BinaryOp::Greater,
					TokenType::GreaterEqual => BinaryOp::GreaterEqual,
					TokenType::Less => BinaryOp::Less,
					_ => BinaryOp::LessEqual,
				},
				right,
			);
		}

		Ok(expr)
	}

	/// term => factor (("+" | "-") factor)*
	fn term(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.factor()?;

		while self.matches_any(&[TokenType::Plus, TokenType::Minus]) {
			let typ = self.previous().typ;
			let right = Box::new(self.factor()?);

			expr = if typ == TokenType::Plus {
				Expr::Binary(Box::new(expr), BinaryOp::Add, right)
			} else {
				Expr::Binary(Box::new(expr), BinaryOp::Sub, right)
			};
		}

		Ok(expr)
	}

	/// factor => unary (("/" | "*") unary)*
	fn factor(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.unary()?;

		while self.matches_any(&[TokenType::Star, TokenType::Slash]) {
			let typ = self.previous().typ;
			let right = Box::new(self.unary()?);

			expr = if typ == TokenType::Star {
				Expr::Binary(Box::new(expr), BinaryOp::Mul, right)
			} else {
				Expr::Binary(Box::new(expr), BinaryOp::Div, right)
			};
		}

		Ok(expr)
	}

	/// unary => ("!" | "-") unary
	fn unary(&mut self) -> Result<Expr, ParseError> {
		if self.matches_any(&[TokenType::Bang, TokenType::Minus]) {
			let typ = self.previous().typ;
			let right = Box::new(self.unary()?);

			return if typ == TokenType::Bang {
				Ok(Expr::Unary(UnaryOp::Not, right))
			} else {
				Ok(Expr::Unary(UnaryOp::Neg, right))
			};
		}

		self.primary()
	}

	/// primary => "(" expression ")", IDENTIFIER, NUMBER | STRING | "true" | "false"
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

		if self.matches(TokenType::Identifier) {
			return Ok(Expr::Var(self.previous().lexeme.clone()));
		}

		ParseError::token_mismatch(
			self.advance(),
			"Expected expression, identifier, number, string, `true` or `false`",
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
