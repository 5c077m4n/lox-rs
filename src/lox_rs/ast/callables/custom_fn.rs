use std::fmt;

use anyhow::{bail, Result};

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
		let Self {
			name: _name,
			inputs,
			body,
		} = self;

		let fn_env = Env::new(Box::new(interp.global.clone()));
		let mut fn_env = Box::new(fn_env);

		for (index, input) in inputs.iter().enumerate() {
			let Expr::Variable(input_name) = input else {
				bail!("Unexpected expression {:?} (should of been a variable)", &input);
 			};
			let input_name = input_name.to_string();
			let input = args.get(index).unwrap_or(&Expr::Literal(Literal::Null));
			let input = interp.expr(input)?;

			fn_env.define(input_name, input);
		}
		interp.exec_block(body, fn_env)
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
