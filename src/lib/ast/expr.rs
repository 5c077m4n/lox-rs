use std::fmt;

use super::{super::lexer::tokens::token_type::Operator, visitor_fn::parenthesize};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expr<'e> {
	Binary {
		left: &'e Expr<'e>,
		op: Operator,
		right: &'e Expr<'e>,
	},
	Grouping {
		expr: &'e Expr<'e>,
	},
	Literal {
		value: Literal,
	},
	Unary {
		op: Operator,
		right: &'e Expr<'e>,
	},
}
impl<'a> Expr<'a> {
	fn dump(&self) {
		println!("{}", parenthesize(self));
	}
}
