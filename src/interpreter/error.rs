#[derive(Debug)]
pub enum RuntimeError {
	ForbiddenType(String),
}

impl RuntimeError {
	pub fn forbidden_type<T>(msg: &str) -> Result<T, Self> {
		Err(Self::ForbiddenType(msg.into()))
	}
}
