use anyhow::Result;

use super::super::{
	super::ast::visitors::interp::Interperter,
	expr::{Expr, Literal},
};

pub trait Callable {
	fn arity(&self) -> usize;
	fn call(&self, _interp: &mut Interperter, args: Vec<Expr>) -> Result<Literal>;
	fn to_string(&self) -> String;
}
