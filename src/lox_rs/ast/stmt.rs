use anyhow::Result;

use super::{
	expr::{Expr, Literal},
	visitors::interp::INTERPERTER,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Expression(Expr),
	Print(Expr),
	Var(String, Option<Expr>),
	Block(Vec<Stmt>),
}
impl Stmt {
	pub fn interpret(self) -> Result<Literal> {
		let mut interp = INTERPERTER.lock().unwrap();
		interp.stmt(self)
	}
}
