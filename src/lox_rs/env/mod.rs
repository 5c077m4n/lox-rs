use std::{collections::BTreeMap, marker::PhantomData};

use anyhow::{anyhow, Result};

use super::ast::expr::Literal;

#[derive(Default, Debug)]
pub struct Env<'e>(BTreeMap<String, Literal>, PhantomData<&'e ()>);

impl Env<'_> {
	pub fn get(&self, name: String) -> Result<&Literal> {
		self.0
			.get(&name)
			.ok_or_else(|| anyhow!("Undefined variable `{}`", name))
	}
	pub fn define(&mut self, name: String, value: Literal) {
		self.0.insert(name, value);
	}
}
