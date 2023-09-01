use super::lexer::tokens::token::Token;

pub struct Expr<'e> {
	left: Box<Expr<'e>>,
	op: Token<'e>,
	right: Box<Expr<'e>>,
}
impl<'e> Expr<'e> {
	pub fn new(left: Box<Expr<'e>>, op: Token<'e>, right: Box<Expr<'e>>) -> Self {
		Self { left, op, right }
	}
}
