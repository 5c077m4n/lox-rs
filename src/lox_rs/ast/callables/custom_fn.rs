use std::fmt;

use anyhow::{bail, Result};

use super::{
	super::{
		super::{
			ast::{expr::Expr, stmt::Stmt, visitors::interp::Interperter},
			env::Env,
		},
		expr::Literal,
	},
	callable::Callable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFn {
	name: String,
	inputs: Vec<Expr>,
	body: Box<Stmt>,
	closure: Box<Env>,
}
impl fmt::Display for CustomFn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self {
			name,
			inputs,
			body,
			closure: _,
		} = self;
		write!(
			f,
			"function {name}({inputs:?}) {{
                {body:#?}
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
			name,
			inputs,
			body,
			closure,
		} = self;

		let mut closure = closure.clone();
		closure.set_parent(Box::new(interp.global.clone()));

		for (index, input) in inputs.iter().enumerate() {
			let Expr::Variable(input_name) = input else {
				bail!(
					"Unexpected expression {:?} (should of been a variable)",
					&input
				);
			};
			let input_name = input_name.to_string();
			let input = args
				.get(index)
				.unwrap_or_else(|| &Expr::Literal(Literal::Null));
			let input = interp.expr(input)?;

			closure.define(input_name, input);
		}
		closure.define(name.to_string(), Literal::CustomFunction(self.clone()));
		log::debug!("{:?}", &closure);

		interp.exec_block(body.as_ref(), closure)
	}
	fn to_string(&self) -> String {
		format!("{self}")
	}
}
impl CustomFn {
	pub fn new(
		name: String,
		inputs: Vec<Expr>,
		body: Box<Stmt>,
		closure: Option<Box<Env>>,
	) -> Self {
		Self {
			name,
			inputs,
			body,
			closure: closure.unwrap_or_default(),
		}
	}
}
