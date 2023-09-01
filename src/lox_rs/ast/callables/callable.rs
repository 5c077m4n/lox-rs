use std::fmt;

use anyhow::Result;

use super::super::expr::Literal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Callable {
	pub arity: usize,
	pub as_str: &'static str,
	pub func: fn(Vec<Literal>) -> Result<Literal>,
}

impl Callable {
	pub fn call(&self, args: Vec<Literal>) -> Result<Literal> {
		(self.func)(args)
	}
}
impl fmt::Display for Callable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.as_str)
	}
}
