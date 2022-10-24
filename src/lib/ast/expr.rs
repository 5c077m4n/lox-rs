use std::fmt;

use super::{super::lexer::tokens::token_type::Operator, visitor_fn::parenthesize};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expr {
	Binary {
		left: Box<Expr>,
		op: Operator,
		right: Box<Expr>,
	},
	Grouping {
		expr: Box<Expr>,
	},
	Literal {
		value: Literal,
	},
	Unary {
		op: Operator,
		right: Box<Expr>,
	},
}
impl Expr {
	pub fn dump(&self) {
		println!("{}", parenthesize(self));
	}
}
