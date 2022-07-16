use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
	pub msg: String,
	pub line: usize,
	// pub col: u64,
}

impl Error {
	pub fn new(msg: String, line: usize) -> Self {
		Self { msg, line }
	}
}

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
	// And,
	// Struct,
	// Else,
	// False,
	// Fun,
	// For,
	// If,
	// Nil,
	// Or,
	// Print,
	// Return,
	// Super,
	// This,
	// True,
	// Let,
	// While,
	Eof,
}

#[derive(Debug)]
pub struct Token {
	pub typ: TokenType,
	lexeme: Vec<char>,
	literal: Option<Literal>,
	_line: usize,
	// col: u64,
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.typ)?;
		if let TokenType::Identifier = self.typ {
			write!(f, "({})", self.lexeme.iter().collect::<String>())?
		}
		if let Some(value) = &self.literal {
			write!(f, "({})", value)?
		}
		Ok(())
	}
}

#[derive(Debug)]
enum Literal {
	String(String),
	Number(f64),
	Boolean(bool),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Literal::String(value) => write!(f, "{value}"),
			Literal::Number(value) => write!(f, "{value}"),
			Literal::Boolean(value) => write!(f, "{value}"),
		}
	}
}

pub struct Lexer {
	// TODO: Should I use a Vec<u8> insted of Vec<char>?
	source: Vec<char>,
	tokens: Vec<Token>,
	errors: Vec<Error>,
	line: usize,
	start: usize,
	current: usize,
}

impl Default for Lexer {
	fn default() -> Self {
		Self {
			source: Vec::new(),
			tokens: Vec::new(),
			errors: Vec::new(),
			line: 1,
			start: 0,
			current: 0,
		}
	}
}

impl Lexer {
	pub fn scan(&mut self, input: String) -> Result<&[Token], &[Error]> {
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
			lexeme: vec![],
			literal: None,
			_line: self.line,
		});

		if self.errors.is_empty() {
			Ok(&self.tokens)
		} else {
			Err(&self.errors)
		}
	}

	fn scan_token(&mut self) -> Result<(), Error> {
		let char = self.advance();
		match char {
			'\n' => {
				self.push_token(TokenType::NewLine, None);
				self.line += 1;
			}
			'\t' | '\r' | ' ' => {}
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
				} else {
					self.push_token(TokenType::Slash, None);
				}
			}
			'"' => self.string(),
			'0'..='9' => self.number(),
			c if c.is_alphabetic() => self.idetifier(),
			c => return Err(Error::new(format!("Unexpected character `{c}`"), self.line)),
		};
		Ok(())
	}

	fn idetifier(&mut self) {
		while {
			let c = self.peek();
			c.is_alphanumeric() || c == '_'
		} {
			self.advance();
		}

		self.push_token(TokenType::Identifier, None)
	}

	fn push_token(&mut self, typ: TokenType, literal: Option<Literal>) {
		self.tokens.push(Token {
			typ,
			literal,
			lexeme: self.source[self.start..self.current].to_vec(),
			_line: self.line,
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
			self
				.errors
				.push(Error::new("Unterminated string!".into(), line_at_start));
			return;
		}

		// Consume closing "
		self.advance();

		let value: String = self.source[self.start + 1..self.current - 1]
			.iter()
			.map(|c| c.to_string())
			.collect();

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
				.push(Error::new(format!("{:?}", err), self.line)),
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