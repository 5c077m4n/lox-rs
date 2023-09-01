use std::collections::{btree_map::Entry, BTreeMap};

use anyhow::{bail, Result};

use super::ast::expr::Literal;

#[derive(Default, Debug, Clone)]
pub struct Env {
	values: BTreeMap<String, Literal>,
	parent: Option<Box<Env>>,
}
impl Env {
	pub fn new(parent: Box<Env>) -> Self {
		Self {
			values: Default::default(),
			parent: Some(parent),
		}
	}
	pub fn add_parent(&mut self, parent: Env) {
		let parent = Box::new(parent);
		self.parent = Some(parent);
	}
	pub fn get(&self, name: String) -> Result<&Literal> {
		if let Some(value) = self.values.get(&name) {
			Ok(value)
		} else if let Some(parent_env) = &self.parent {
			parent_env.get(name)
		} else {
			bail!("Undefined variable `{}`", name);
		}
	}
	pub fn get_parent(&self) -> Option<Box<Self>> {
		self.parent.clone()
	}
	pub fn define(&mut self, name: String, value: Literal) {
		self.values.insert(name, value);
	}
	pub fn redefine(&mut self, name: String, value: Literal) -> Result<()> {
		if let Entry::Occupied(mut e) = self.values.entry(name.clone()) {
			e.insert(value);
			Ok(())
		} else if let Some(parent_env) = &mut self.parent {
			parent_env.redefine(name, value)
		} else {
			bail!("{} was not initiated yet", &name);
		}
	}
	pub fn remove(&mut self, key: &str) -> Option<Literal> {
		self.values.remove(key)
	}
}
