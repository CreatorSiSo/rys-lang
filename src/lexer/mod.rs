use crate::literal::Literal;
use crate::token::{Token, TokenType};

mod error;
mod unescape;
use error::LexerError;
use unescape::unescape;

const KEYWORDS: [(&str, TokenType); 17] = [
	("and", TokenType::And),
	("struct", TokenType::Struct),
	("else", TokenType::Else),
	("false", TokenType::False),
	("for", TokenType::For),
	("fun", TokenType::Fun),
	("if", TokenType::If),
	("or", TokenType::Or),
	("print", TokenType::Print),
	("return", TokenType::Return),
	("super", TokenType::Super),
	("this", TokenType::This),
	("true", TokenType::True),
	("while", TokenType::While),
	("loop", TokenType::Loop),
	("const", TokenType::Const),
	("mut", TokenType::Mut),
];

pub struct Lexer {
	// TODO: Should I use a Vec<u8> insted of Vec<char>?
	source: Vec<char>,
	tokens: Vec<Token>,
	errors: Vec<error::LexerError>,
	line: usize,
	start: usize,
	current: usize,
}

impl Lexer {
	pub fn new() -> Self {
		Self {
			source: Vec::new(),
			tokens: Vec::new(),
			errors: Vec::new(),
			line: 1,
			start: 0,
			current: 0,
		}
	}

	pub fn scan(&mut self, input: String) -> Result<&[Token], &[LexerError]> {
		self.errors.clear();
		self.line = 1;
		self.source = input.chars().collect();

		while !self.is_at_end() {
			self.start = self.current;
			if let Err(err) = self.scan_token() {
				self.errors.push(err)
			}
		}

		self.tokens.push(Token {
			typ: TokenType::Eof,
			lexeme: String::new(),
			literal: None,
			line: self.line,
		});

		if self.errors.is_empty() {
			Ok(&self.tokens)
		} else {
			Err(&self.errors)
		}
	}

	fn scan_token(&mut self) -> Result<(), LexerError> {
		let char = self.advance();
		match char {
			'\n' => {
				// TODO: Make it possible to use `\n` insted of `;`
				// self.push_token(TokenType::NewLine, None);
				self.line += 1;
			}
			'\t' | '\r' | ' ' => { /* Ignore whitespace */ }
			';' => self.push_token(TokenType::Semicolon, None),
			'(' => self.push_token(TokenType::LeftParen, None),
			')' => self.push_token(TokenType::RightParen, None),
			'{' => self.push_token(TokenType::LeftBrace, None),
			'}' => self.push_token(TokenType::RightBrace, None),
			',' => self.push_token(TokenType::Comma, None),
			'.' => self.push_token(TokenType::Dot, None),
			'-' => self.push_token(TokenType::Minus, None),
			'+' => self.push_token(TokenType::Plus, None),
			'*' => self.push_token(TokenType::Star, None),
			'!' if self.matches('=') => self.push_token(TokenType::BangEqual, None),
			'!' => self.push_token(TokenType::Bang, None),
			'=' if self.matches('=') => self.push_token(TokenType::EqualEqual, None),
			'=' => self.push_token(TokenType::Equal, None),
			'<' if self.matches('=') => self.push_token(TokenType::LessEqual, None),
			'<' => self.push_token(TokenType::Less, None),
			'>' if self.matches('=') => self.push_token(TokenType::GreaterEqual, None),
			'>' => self.push_token(TokenType::Greater, None),
			'/' => {
				if self.matches('/') {
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				} else if self.matches('*') {
					let mut nested = 0;
					while !self.is_at_end() {
						if self.peek() == '\n' {
							self.line += 1;
						} else if self.peek() == '/' && self.peek_next() == '*' {
							nested += 1;
						} else if self.peek() == '*' && self.peek_next() == '/' {
							nested -= 1;
						}
						if nested < 0 {
							break;
						}
						self.advance();
					}

					if !self.is_at_end() {
						// Consume */
						self.advance();
						self.advance();
					}
				} else {
					self.push_token(TokenType::Slash, None);
				}
			}
			'"' => self.string(),
			'0'..='9' => self.number(),
			c if c.is_alphabetic() => self.identifier(),
			c => {
				return Err(LexerError::new(
					format!("Unexpected character `{c}`"),
					self.line,
				))
			}
		};
		Ok(())
	}

	fn identifier(&mut self) {
		while {
			let c = self.peek();
			c.is_alphanumeric() || c == '_'
		} {
			self.advance();
		}

		let text: String = self.source[self.start..self.current]
			.iter()
			.map(|c| c.to_string())
			.collect();

		match KEYWORDS
			.iter()
			.find_map(|(key, token_type)| if key == &text { Some(token_type) } else { None })
		{
			Some(TokenType::True) => self.push_token(TokenType::True, Some(Literal::True)),
			Some(TokenType::False) => self.push_token(TokenType::True, Some(Literal::False)),
			Some(token_type) => self.push_token(*token_type, None),
			None => self.push_token(TokenType::Identifier, None),
		}
	}

	fn push_token(&mut self, typ: TokenType, literal: Option<Literal>) {
		self.tokens.push(Token {
			typ,
			literal,
			lexeme: self.source[self.start..self.current].iter().collect(),
			line: self.line,
		})
	}

	fn string(&mut self) {
		let line_at_start = self.line;

		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1
			}
			self.advance();
		}

		if self.is_at_end() {
			self.errors.push(LexerError::new(
				"Unterminated string!".into(),
				line_at_start,
			));
			return;
		}

		// Consume closing "
		self.advance();

		let slice = &self.source[self.start + 1..self.current - 1];
		let value = unescape(slice);
		self.push_token(TokenType::String, Some(Literal::String(value)));
	}

	fn number(&mut self) {
		while self.peek().is_ascii_digit() && !self.is_at_end() {
			self.advance();
		}

		if self.peek() == '.' && self.peek_next().is_ascii_digit() {
			// Consume .
			self.advance();

			while self.peek().is_ascii_digit() {
				self.advance();
			}
		}

		let value: String = self.source[self.start..self.current]
			.iter()
			.map(|c| c.to_string())
			.collect();

		match value.parse::<f64>() {
			Ok(number) => self.push_token(TokenType::Number, Some(Literal::Number(number))),
			Err(err) => self
				.errors
				.push(LexerError::new(format!("{:?}", err), self.line)),
		}
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		self.source[self.current - 1]
	}

	fn matches(&mut self, char: char) -> bool {
		if self.is_at_end() {
			return false;
		};
		if self.source[self.current] != char {
			return false;
		};

		self.current += 1;
		true
	}

	fn peek(&self) -> char {
		if self.is_at_end() {
			'\0'
		} else {
			self.source[self.current]
		}
	}

	fn peek_next(&self) -> char {
		if self.current + 1 >= self.source.len() {
			'\0'
		} else {
			self.source[self.current + 1]
		}
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}
}
