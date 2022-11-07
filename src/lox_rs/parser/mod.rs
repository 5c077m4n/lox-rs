use std::iter::Peekable;

use anyhow::{anyhow, bail, Ok, Result};

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
		let mut parser = Parser {
			tokens,
			history: Vec::new(),
			errors: Vec::new(),
		};
		parser.advance();
		parser
	}

	fn get_token_at(&self, rel: usize) -> Result<&TokenType> {
		let pos = self.history.len().saturating_sub(rel).saturating_sub(1);
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
	fn advance(&mut self) {
		if let Some(token) = self.tokens.next() {
			self.history.push(token);
		}
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
				self.advance();
				return Ok(Some(t));
			}
		}
		Ok(None)
	}

	fn expression(&mut self) -> Result<Expr> {
		self.equality()
	}
	fn equality(&mut self) -> Result<Expr> {
		use token_type::Operator;

		let mut expr = self.comparison()?;

		while let TokenType::Operator(op @ (Operator::EqEq | Operator::NotEq)) = self.current()? {
			let op = op.clone();
			self.advance();

			let right = self.comparison()?;

			expr = Expr::Binary(Box::new(expr), op, Box::new(right));
		}
		Ok(expr)
	}
	fn comparison(&mut self) -> Result<Expr> {
		use token_type::Operator;

		let mut expr = self.term()?;

		while let TokenType::Operator(
			op @ (Operator::Gt | Operator::Gte | Operator::Lt | Operator::Lte),
		) = self.current()?
		{
			let op = op.clone();
			self.advance();

			let right = self.term()?;

			expr = Expr::Binary(Box::new(expr), op, Box::new(right));
		}

		Ok(expr)
	}
	fn term(&mut self) -> Result<Expr> {
		use token_type::Operator;

		let mut expr = self.factor()?;

		while let TokenType::Operator(op @ (Operator::Add | Operator::Sub)) = self.current()? {
			let op = op.clone();
			self.advance();

			let right = self.factor()?;

			expr = Expr::Binary(Box::new(expr), op, Box::new(right));
		}
		Ok(expr)
	}
	fn factor(&mut self) -> Result<Expr> {
		use token_type::Operator;

		let mut expr = self.unary()?;

		while let TokenType::Operator(op @ (Operator::Mul | Operator::Div)) = self.current()? {
			let op = op.clone();
			self.advance();

			let right = self.unary()?;

			expr = Expr::Binary(Box::new(expr), op, Box::new(right));
		}
		Ok(expr)
	}
	fn unary(&mut self) -> Result<Expr> {
		use token_type::Operator;

		if let TokenType::Operator(op @ (Operator::Not | Operator::Sub | Operator::Add)) =
			self.current()?
		{
			let op = op.clone();
			self.advance();

			let right = self.unary()?;

			Ok(Expr::Unary(op, Box::new(right)))
		} else {
			self.primary()
		}
	}
	fn primary(&mut self) -> Result<Expr> {
		match self.current()? {
			TokenType::Literal(lit) => {
				let value = match lit {
					token_type::Literal::String(v) => {
						Literal::String(String::from_utf8(v.to_vec())?)
					}
					token_type::Literal::Number(v) => Literal::Number(*v),
					token_type::Literal::Boolean(v) => Literal::Boolean(*v),
					token_type::Literal::Null => Literal::Null,
				};

				self.advance();
				Ok(Expr::Literal(value))
			}
			TokenType::Punctuation(token_type::Punctuation::BracketOpen) => {
				self.advance();

				let expr = self.expression()?;
				self.consume(
					&TokenType::Punctuation(token_type::Punctuation::BracketClose),
					"Expected a `)` after the expression",
				)?;

				Ok(Expr::Grouping(Box::new(expr)))
			}
			other => {
				// FIXME: A result should be retruned here to not break flow
				bail!("Unknown primary expression received, but got {:?}", &other)
			}
		}
	}
	fn consume(&mut self, until_token: &TokenType, err_msg: &'p str) -> Result<()> {
		if self.check(until_token)? {
			self.advance();
		} else {
			self.errors.push(err_msg);
		}
		Ok(())
	}
	fn sync(&mut self) -> Result<()> {
		use token_type::{Keyword, Punctuation};

		self.advance();

		while !self.is_at_end() {
			let prev = self.prev()?;
			let current = self.current()?;

			if prev != &TokenType::Punctuation(Punctuation::Semicolon)
				&& current != &TokenType::Keyword(Keyword::Class)
				&& current != &TokenType::Keyword(Keyword::Function)
				&& current != &TokenType::Keyword(Keyword::Var)
				&& current != &TokenType::Keyword(Keyword::For)
				&& current != &TokenType::Keyword(Keyword::If)
				&& current != &TokenType::Keyword(Keyword::While)
				&& current != &TokenType::Keyword(Keyword::Print)
				&& current != &TokenType::Keyword(Keyword::Return)
			{
				self.advance();
			}
		}
		Ok(())
	}
	pub fn parse(&mut self) -> Result<(Expr, &[&'p str])> {
		self.expression().map(|expr| (expr, &self.errors[..]))
	}
}

#[cfg(test)]
mod tests;
