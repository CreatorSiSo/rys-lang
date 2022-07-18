use crate::literal::Literal;
use std::collections::HashMap;

use super::error::RuntimeError;

pub struct Env(HashMap<String, EnvValue>);

pub struct EnvValue {
	value: Literal,
	mutable: bool,
}

impl Env {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn get(&self, name: &str) -> Result<&Literal, RuntimeError> {
		match self.0.get(name) {
			Some(EnvValue { value, .. }) => Ok(value),
			None => RuntimeError::undeclared_var(name),
		}
	}

	pub fn set(&mut self, name: &str, new: Literal) -> Result<(), RuntimeError> {
		match self.0.get_mut(name) {
			Some(var) => {
				if var.mutable == false {
					return RuntimeError::assignment(name, new);
				}
				var.value = new;
				Ok(())
			}
			None => RuntimeError::undeclared_var(name),
		}
	}

	pub fn declare(&mut self, name: String, value: Literal, mutable: bool) {
		self.0.insert(name, EnvValue { value, mutable });
	}
}
