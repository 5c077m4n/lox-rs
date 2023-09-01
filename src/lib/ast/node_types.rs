use super::{super::lexer::tokens::token::Token, visitor::Visitor};

pub trait AcceptVisitor {
	fn accept(&self, visitor: &dyn Visitor);
}

pub struct Expr<'e> {
	left: &'e Expr<'e>,
	op: Token<'e>,
	right: &'e Expr<'e>,
}
impl<'e> Expr<'e> {
	pub fn new(left: &'e Expr<'e>, op: Token<'e>, right: &'e Expr<'e>) -> Self {
		Self { left, op, right }
	}
}

pub struct Binary<'b> {
	left: &'b Expr<'b>,
	op: Token<'b>,
	right: &'b Expr<'b>,
}
impl AcceptVisitor for Binary<'_> {
	fn accept(&self, visitor: &dyn Visitor) {
		visitor.visit_binary(self)
	}
}

pub struct Grouping<'g> {
	expr: &'g Expr<'g>,
}
impl AcceptVisitor for Grouping<'_> {
	fn accept(&self, visitor: &dyn Visitor) {
		visitor.visit_grouping(self)
	}
}

pub struct Unary<'u> {
	op: Token<'u>,
	right: &'u Expr<'u>,
}
impl AcceptVisitor for Unary<'_> {
	fn accept(&self, visitor: &dyn Visitor) {
		visitor.visit_unary(self)
	}
}
