use super::error::RuntimeError;
use crate::literal::Literal;
use std::collections::HashMap;

pub struct Variable {
	value: Literal,
	mutable: bool,
}

type Scope = HashMap<String, Variable>;

pub struct Env {
	scopes: Vec<Scope>,
}

impl Env {
	pub fn new() -> Self {
		Self {
			scopes: vec![Scope::new()],
		}
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new())
	}

	pub fn pop_scope(&mut self) {
		if self.scopes.len() > 1 {
			self.scopes.pop();
		}
	}

	pub fn get(&self, name: &str) -> Result<&Literal, RuntimeError> {
		for scope in self.iter() {
			match scope.get(name) {
				Some(var) => return Ok(&var.value),
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub fn set(&mut self, name: &str, new: Literal) -> Result<(), RuntimeError> {
		for scope in self.iter_mut() {
			match scope.get_mut(name) {
				Some(var) => {
					if !var.mutable {
						return RuntimeError::assignment(name, new);
					}
					var.value = new;
					return Ok(());
				}
				None => continue,
			}
		}
		RuntimeError::undeclared_var(name)
	}

	pub fn declare(&mut self, name: String, value: Literal, mutable: bool) {
		self.last_mut().insert(name, Variable { value, mutable });
	}
}

impl Env {
	fn last_mut(&mut self) -> &mut Scope {
		self
			.scopes
			.last_mut()
			.expect("Internal Error: Stack should never be empty!")
	}

	fn iter(&self) -> std::iter::Rev<std::slice::Iter<Scope>> {
		self.scopes.iter().rev()
	}

	fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<Scope>> {
		self.scopes.iter_mut().rev()
	}
}
