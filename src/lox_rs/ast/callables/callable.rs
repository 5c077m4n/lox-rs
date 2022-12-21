use std::fmt;

use anyhow::Result;

use super::super::{super::ast::visitors::interp::Interperter, expr::Literal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Callable {
	arity: usize,
	as_string: String,
	func: fn(Vec<Literal>) -> Result<Literal>,
}

impl Callable {
	pub fn new(arity: usize, as_string: String, func: fn(Vec<Literal>) -> Result<Literal>) -> Self {
		Self {
			arity,
			as_string,
			func,
		}
	}
	pub fn arity(&self) -> usize {
		self.arity
	}
	pub fn call(&self, _interp: &Interperter, args: Vec<Literal>) -> Result<Literal> {
		(self.func)(args)
	}
}
impl fmt::Display for Callable {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.as_string)
	}
}
