use std::fmt;

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
	name: &'static str,
	arity: usize,
	func: fn(Vec<Literal>) -> Result<Literal>,
}
impl fmt::Display for NativeFn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { name, .. } = self;
		write!(f, "<native fn `{name}`>",)
	}
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
		format!("{self}")
	}
}
impl NativeFn {
	pub const fn new(
		name: &'static str,
		arity: usize,
		func: fn(Vec<Literal>) -> Result<Literal>,
	) -> Self {
		Self { name, arity, func }
	}

	pub fn get_name(&self) -> String {
		self.name.to_string()
	}
}
