#[derive(Debug)]
pub struct LexerError {
	pub msg: String,
	pub line: usize,
	// pub col: u64,
}

impl LexerError {
	pub fn new(msg: String, line: usize) -> Self {
		Self { msg, line }
	}
}
