use anyhow::Result;

use super::{
	super::{super::ast::visitors::interp::Interperter, expr::Literal},
	callable::Callable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFn {
	inputs: Vec<Literal>,
	as_string: String,
}
impl Callable for CustomFn {
	fn arity(&self) -> usize {
		self.inputs.len()
	}
	fn call(&self, _interp: &Interperter, _args: Vec<Literal>) -> Result<Literal> {
		Ok(Literal::Null)
	}
	fn to_string(&self) -> String {
		self.as_string.to_owned()
	}
}
impl CustomFn {
	pub fn new(inputs: Vec<Literal>, as_string: String) -> Self {
		Self { inputs, as_string }
	}
}
