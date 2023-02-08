use anyhow::Result;

use super::{
	super::{super::ast::visitors::interp::Interperter, expr::Literal},
	callable::Callable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFn {
	arity: usize,
	func: fn(Vec<Literal>) -> Result<Literal>,
}
impl Callable for NativeFn {
	fn arity(&self) -> usize {
		self.arity
	}
	fn call(&self, _interp: &Interperter, args: Vec<Literal>) -> Result<Literal> {
		(self.func)(args)
	}
	fn to_string(&self) -> String {
		"<native fn>".to_string()
	}
}
impl NativeFn {
	pub const fn new(arity: usize, func: fn(Vec<Literal>) -> Result<Literal>) -> Self {
		Self { arity, func }
	}
}
