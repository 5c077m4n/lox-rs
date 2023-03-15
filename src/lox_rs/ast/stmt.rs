use anyhow::Result;

use super::{
	expr::{Expr, Literal},
	visitors::interp::Interperter,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Expression(Expr),
	Print(Expr),
	Var(String, Option<Expr>),
	Block(Vec<Stmt>),
	If(Expr, Box<Stmt>, Option<Box<Stmt>>),
	While(Expr, Box<Stmt>),
	For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
	Function(String, Vec<Expr>, Box<Stmt>),
	Return(Expr),
}
impl Stmt {
	pub fn interpret(&self, interp: &mut Interperter) -> Result<Literal> {
		interp.stmt(self)
	}
}
