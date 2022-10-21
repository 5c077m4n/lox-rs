use super::super::lexer::tokens::token::Token;

#[derive(Debug)]
pub enum Literal {
	Number(f64),
	String(String),
	Boolean(bool),
	Null,
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
	fn visit(&self) {
		match self {
			Expr::Binary { .. } => todo!(),
			Expr::Grouping { .. } => todo!(),
			Expr::Literal { .. } => todo!(),
			Expr::Unary { .. } => todo!(),
		}
	}
}
