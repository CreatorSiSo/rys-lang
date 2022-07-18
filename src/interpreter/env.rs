use crate::literal::Literal;
use std::collections::HashMap;

use super::error::RuntimeError;

pub struct Env(HashMap<String, Literal>);

impl Env {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn get(&self, name: &str) -> Result<&Literal, RuntimeError> {
		match self.0.get(name) {
			Some(value) => Ok(value),
			None => RuntimeError::undeclared_var(name),
		}
	}

	pub fn declare(&mut self, name: String, value: Literal) {
		self.0.insert(name, value);
	}
}
