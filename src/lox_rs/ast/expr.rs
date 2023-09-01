use std::fmt;

use anyhow::Result;

use super::{
	super::lexer::tokens::token_type::Operator,
	visitors::{interp::INTERPERTER, parens::parenthesize},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Number(f64),
	String(String),
	Boolean(bool),
	Null,
}
impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Literal::Number(n) => write!(f, "{}", n),
			Literal::String(s) => write!(f, "{}", s),
			Literal::Boolean(b) => write!(f, "{}", b),
			Literal::Null => write!(f, "null"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Assign(String, Box<Expr>),
	Binary(Box<Expr>, Operator, Box<Expr>),
	Grouping(Box<Expr>),
	Literal(Literal),
	Unary(Operator, Box<Expr>),
	Variable(String),
}
impl Expr {
	pub fn interpret(self) -> Result<Literal> {
		let mut interp = INTERPERTER.lock().unwrap();
		interp.expr(self)
	}
}
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &parenthesize(self))
	}
}
