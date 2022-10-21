use super::{super::lexer::tokens::token::Token, visitor_fn::parenthesize};

#[derive(Debug)]
pub enum Literal {
	Number(f64),
	String(String),
	Boolean(bool),
	Null,
}
impl ToString for Literal {
	fn to_string(&self) -> String {
		match self {
			Literal::Number(n) => n.to_string(),
			Literal::String(s) => s.to_owned(),
			Literal::Boolean(b) => b.to_string(),
			Literal::Null => "null".to_string(),
		}
	}
}

#[derive(Debug)]
pub enum Expr<'e> {
	Binary {
		left: &'e Expr<'e>,
		op: Token<'e>,
		right: &'e Expr<'e>,
	},
	Grouping {
		expr: &'e Expr<'e>,
	},
	Literal {
		value: Literal,
	},
	Unary {
		op: Token<'e>,
		right: &'e Expr<'e>,
	},
}

impl<'a> Expr<'a> {
	fn dump(&self) {
		println!("{}", parenthesize(self));
	}
}
