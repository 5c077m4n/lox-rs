use std::fmt;

use anyhow::Result;

use super::{
	super::lexer::tokens::token_type::Operator,
	visitors::{
		interp::{interpret_expr, interpret_stmt},
		parens::parenthesize,
	},
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
	Binary(Box<Expr>, Operator, Box<Expr>),
	Grouping(Box<Expr>),
	Literal(Literal),
	Unary(Operator, Box<Expr>),
}
impl Expr {
	pub fn interpret(&self) -> Result<Literal> {
		interpret_expr(self)
	}
}
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &parenthesize(self))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Expression(Expr),
	Print(Expr),
}
impl Stmt {
	pub fn interpret(&self) -> Result<Literal> {
		interpret_stmt(self)
	}
}
