use std::iter::Peekable;

use anyhow::{anyhow, bail, Result};

use super::{
	ast::expr::{Expr, Literal},
	lexer::tokens::{
		token::Token,
		token_type::{self, TokenType},
	},
};

pub struct Parser<'p, I: Iterator<Item = Token<'p>>> {
	tokens: Box<Peekable<I>>,
	history: Vec<Token<'p>>,
	errors: Vec<&'p str>,
}
impl<'p, I: Iterator<Item = Token<'p>>> Parser<'p, I> {
	pub fn new(tokens: Box<Peekable<I>>) -> Self {
		Parser {
			tokens,
			history: Vec::new(),
			errors: Vec::new(),
		}
	}
	fn get_token_at(&self, rel: usize) -> Result<&TokenType> {
		let pos = self.history.len().saturating_sub(rel);
		let token = self
			.history
			.get(pos)
			.ok_or_else(|| anyhow!("There should be a token @ {}", pos))?
			.get();
		Ok(token)
	}
	fn is_at_end(&self) -> bool {
		if let Some(token) = self.history.last() {
			token.get() == &TokenType::EndOfFile
		} else {
			true
		}
	}
	/// Advance the current index if not at the EOF yet
	fn advance(&mut self) -> Result<()> {
		if let Some(token) = self.tokens.next() {
			self.history.push(token);
		}
		Ok(())
	}
	/// Get current token
	fn current(&self) -> Result<&TokenType> {
		self.get_token_at(0)
	}
	/// Get previous token
	fn prev(&self) -> Result<&TokenType> {
		self.get_token_at(1)
	}
	/// Check if the current token is of a give type
	fn check(&self, token: &TokenType) -> Result<bool> {
		Ok(!self.is_at_end() && self.current()? == token)
	}
	/// Match the current token against a given list and advance the index only if there is a match
	fn match_type(&mut self, types: &'p [&TokenType]) -> Result<Option<&TokenType>> {
		for t in types {
			if self.check(t)? {
				self.advance()?;
				return Ok(Some(t));
			}
		}
		Ok(None)
	}

	fn expression(&mut self) -> Result<Expr> {
		self.equality()
	}
	fn equality(&mut self) -> Result<Expr> {
		let mut expr = self.comparison()?;

		while let Some(op) = self.tokens.next() {
			use token_type::Operator;

			match op.get() {
				TokenType::Operator(op @ (Operator::EqEq | Operator::NotEq)) => {
					let op = op.clone();
					let right = self.comparison()?;

					expr = Expr::Binary {
						left: Box::new(expr),
						op,
						right: Box::new(right),
					};
				}
				_ => unreachable!("How did I get here?"),
			}
		}
		Ok(expr)
	}
	fn comparison(&mut self) -> Result<Expr> {
		let mut expr = self.term()?;

		while let Some(op) = self.tokens.next() {
			use token_type::Operator;

			match op.get() {
				TokenType::Operator(
					op @ (Operator::Gt | Operator::Gte | Operator::Lt | Operator::Lte),
				) => {
					let op = op.clone();
					let right = self.term()?;

					expr = Expr::Binary {
						left: Box::new(expr),
						op,
						right: Box::new(right),
					};
				}
				other => unreachable!(
					"The token should be an operator, but got {:?} - tested above",
					&other
				),
			}
		}
		Ok(expr)
	}
	fn term(&mut self) -> Result<Expr> {
		use token_type::Operator;

		let mut expr = self.factor()?;

		while let Some(op) = self.tokens.next() {
			match op.get() {
				TokenType::Operator(op @ (Operator::Add | Operator::Sub)) => {
					let op = op.clone();
					let right = self.factor()?;

					expr = Expr::Binary {
						left: Box::new(expr),
						op,
						right: Box::new(right),
					};
				}
				other => {
					unreachable!(
						"The token should be an operator, but got {:?} - tested above",
						&other
					)
				}
			}
		}
		Ok(expr)
	}
	fn factor(&mut self) -> Result<Expr> {
		let mut expr = self.unary()?;

		while let Some(op) = self.tokens.next() {
			use token_type::Operator;

			if let TokenType::Operator(op @ (Operator::Mul | Operator::Div)) = op.get() {
				let op = op.clone();
				let right = self.unary()?;

				expr = Expr::Binary {
					left: Box::new(expr),
					op,
					right: Box::new(right),
				};
			} else {
				unreachable!("The token should be an operator - tested above")
			}
		}
		Ok(expr)
	}
	fn unary(&mut self) -> Result<Expr> {
		if let Some(op) = self.tokens.next() {
			use token_type::Operator;

			if let TokenType::Operator(op @ (Operator::Not | Operator::Sub | Operator::Add)) =
				op.get()
			{
				let op = op.clone();
				let right = self.unary()?;

				Ok(Expr::Unary {
					op,
					right: Box::new(right),
				})
			} else {
				unreachable!("The token should be an operator - tested above")
			}
		} else {
			self.primary()
		}
	}
	fn primary(&mut self) -> Result<Expr> {
		let Some(token) = self.tokens.next() else {
			bail!("Expression expected here");
        };
		let token = token.get();

		match token {
			TokenType::Literal(lit) => {
				let value = match lit {
					token_type::Literal::String(v) => {
						Literal::String(String::from_utf8(v.to_vec())?)
					}
					token_type::Literal::Number(v) => Literal::Number(*v),
					token_type::Literal::Boolean(v) => Literal::Boolean(*v),
					token_type::Literal::Null => Literal::Null,
				};
				Ok(Expr::Literal { value })
			}
			TokenType::Punctuation(token_type::Punctuation::BracketOpen) => {
				let expr = self.expression()?;
				self.consume(
					&TokenType::Punctuation(token_type::Punctuation::BracketClose),
					"Expected a `)` after the expression",
				)?;

				Ok(Expr::Grouping {
					expr: Box::new(expr),
				})
			}
			other => {
				// FIXME: A result should be retruned here to not break flow
				bail!("Expression expected here, but got {:?}", &other)
			}
		}
	}
	fn consume(&mut self, until_token: &TokenType, err_msg: &'p str) -> Result<()> {
		if self.check(until_token)? {
			self.advance()?;
		} else {
			self.errors.push(err_msg);
		}
		Ok(())
	}
	fn sync(&mut self) -> Result<()> {
		self.advance()?;

		while !self.is_at_end() {
			let prev = self.prev()?;
			let current = self.current()?;

			if prev != &TokenType::Punctuation(token_type::Punctuation::Semicolon)
				&& current != &TokenType::Keyword(token_type::Keyword::Class)
				&& current != &TokenType::Keyword(token_type::Keyword::Function)
				&& current != &TokenType::Keyword(token_type::Keyword::Var)
				&& current != &TokenType::Keyword(token_type::Keyword::For)
				&& current != &TokenType::Keyword(token_type::Keyword::If)
				&& current != &TokenType::Keyword(token_type::Keyword::While)
				&& current != &TokenType::Keyword(token_type::Keyword::Print)
				&& current != &TokenType::Keyword(token_type::Keyword::Return)
			{
				self.advance()?;
			}
		}
		Ok(())
	}
	pub fn parse(&mut self) -> Result<(Expr, &Vec<&'p str>)> {
		self.expression().map(|expr| (expr, &self.errors))
	}
}
