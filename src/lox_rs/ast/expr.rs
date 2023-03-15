use std::fmt;

use super::{
	super::lexer::tokens::token_type::{Operator, Punctuation},
	callables::{custom_fn::CustomFn, native_fn::NativeFn},
	visitors::parens::parenthesize,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Literal {
	Number(f64),
	String(String),
	Boolean(bool),
	NativeFunction(NativeFn),
	CustomFunction(CustomFn),
	#[default]
	Null,
}
impl Literal {
	pub fn is_truthy(&self) -> bool {
		match self {
			Self::Number(n) => *n != 0.,
			Self::String(s) => !s.is_empty(),
			Self::Boolean(b) => *b,
			Literal::CustomFunction(_) => true,
			Literal::NativeFunction(_) => true,
			Self::Null => false,
		}
	}
}
impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Literal::Number(n) => write!(f, "{n}"),
			Literal::String(s) => write!(f, "\"{s}\""),
			Literal::Boolean(b) => write!(f, "{b}"),
			Literal::CustomFunction(func) => write!(f, "{func}"),
			Literal::NativeFunction(func) => write!(f, "{func}"),
			Literal::Null => write!(f, "null"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Assign(String, Box<Expr>),
	Binary(Box<Expr>, Operator, Box<Expr>),
	Call(Box<Expr>, Punctuation, Vec<Expr>),
	Grouping(Box<Expr>),
	Literal(Literal),
	Unary(Operator, Box<Expr>),
	Variable(String),
	Logical(Box<Expr>, Operator, Box<Expr>),
}
impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &parenthesize(self))
	}
}
