pub(crate) fn unescape(slice: &[char]) -> String {
	let mut iter = slice.iter().peekable();
	let mut result = String::new();
	loop {
		if let Some(c) = iter.next() {
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
		} else {
			break;
		}
	}
	result
}
