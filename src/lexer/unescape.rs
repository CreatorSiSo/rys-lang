pub(crate) fn unescape(slice: &[char]) -> String {
	let mut iter = slice.iter().peekable();
	let mut result = String::new();
	while let Some(c) = iter.next() {
		// TODO: Implement other escape codes
		// https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_lexer/unescape.rs.html
		if *c == '\\' {
			match iter.peek() {
				Some('t') => {
					iter.next();
					result.push('\t');
				}
				Some('n') => {
					iter.next();
					result.push('\n');
				}
				_ => result.push('\\'),
			}
		} else {
			result.push(*c)
		}
	}
	result
}
