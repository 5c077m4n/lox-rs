use anyhow::Result;

use super::{
	super::{
		super::ast::visitors::interp::Interperter,
		expr::{Expr, Literal},
	},
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
	fn call(&self, interp: &mut Interperter, args: Vec<Expr>) -> Result<Literal> {
		let args: Vec<Literal> = args.iter().map(|a| interp.expr(a).unwrap()).collect();
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
