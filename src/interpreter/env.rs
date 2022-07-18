use super::error::RuntimeError;
use crate::literal::Literal;
use std::collections::HashMap;

pub struct Env<'a> {
	parent: Option<&'a mut Env<'a>>,
	variables: HashMap<String, Variable>,
}

pub struct Variable {
	value: Literal,
	mutable: bool,
}

impl<'a> Env<'a> {
	pub fn new() -> Self {
		Self {
			parent: None,
			variables: HashMap::new(),
		}
	}

	pub fn new_scope(parent_env: &'a mut Env<'a>) -> Self {
		Self {
			parent: Some(parent_env),
			variables: HashMap::new(),
		}
	}

	pub fn get(&self, name: &str) -> Result<&Literal, RuntimeError> {
		match self.variables.get(name) {
			Some(Variable { value, .. }) => Ok(value),
			None => match &self.parent {
				Some(parent) => parent.get(name),
				None => RuntimeError::undeclared_var(name),
			},
		}
	}

	pub fn set(&mut self, name: &str, new: Literal) -> Result<(), RuntimeError> {
		match self.variables.get_mut(name) {
			Some(var) => {
				if var.mutable == false {
					return RuntimeError::assignment(name, new);
				}
				var.value = new;
				Ok(())
			}
			None => match &mut self.parent {
				Some(parent) => parent.set(name, new),
				None => RuntimeError::undeclared_var(name),
			},
		}
	}

	pub fn declare(&mut self, name: String, value: Literal, mutable: bool) {
		self.variables.insert(name, Variable { value, mutable });
	}
}
