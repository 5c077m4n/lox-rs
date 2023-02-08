use std::fmt;

use anyhow::Result;

use super::{
	super::{
		super::ast::{expr::Expr, stmt::Stmt, visitors::interp::Interperter},
		expr::Literal,
	},
	callable::Callable,
};
use crate::lox_rs::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFn {
	name: String,
	inputs: Vec<Expr>,
	body: Box<Stmt>,
}
impl fmt::Display for CustomFn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { name, inputs, body } = self;
		write!(
			f,
			"function {name}({inputs:?}) {{
                {body:?}
            }}",
		)
	}
}
impl Callable for CustomFn {
	fn arity(&self) -> usize {
		self.inputs.len()
	}
	fn call(&self, interp: &mut Interperter, args: Vec<Expr>) -> Result<Literal> {
		let Self { name, inputs, body } = self;
		let mut fn_env = Env::new(Box::new(interp.global.clone()));

		for arg in args.iter() {
			let arg = arg.clone();
			let arg = interp.expr(arg)?;
			fn_env.define(arg.to_string(), arg);
		}
		todo!()
	}
	fn to_string(&self) -> String {
		format!("{self}")
	}
}
impl CustomFn {
	pub fn new(name: String, inputs: Vec<Expr>, body: Box<Stmt>) -> Self {
		Self { name, inputs, body }
	}
}
