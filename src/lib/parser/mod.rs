use anyhow::{anyhow, bail, Result};

use super::{
	ast::expr::{Expr, Literal},
	lexer::tokens::{
		token::Token,
		token_type::{self, Operator, Punctuation, TokenType},
	},
};

pub struct Parser<'p> {
	tokens: &'p [Token<'p>],
	index: usize,
}
impl<'p> Parser<'p> {
	fn get_token_at(&self, rel: isize) -> Result<&TokenType> {
		let pos = (self.index as isize) + rel;
		let pos: usize = pos.try_into()?;

		let token = self
			.tokens
			.get(pos)
			.ok_or_else(|| anyhow!("There should be a token @ {}", pos))?
			.get();
		Ok(token)
	}
	fn is_at_end(&self) -> Result<bool> {
		Ok(self.get_token_at(0)? == &TokenType::Punctuation(Punctuation::EndOfFile))
	}
	/// Advance the current index if not at the EOF yet
	fn advance(&mut self) -> Result<()> {
		if !(self.is_at_end()?) {
			self.index += 1;
		}
		Ok(())
	}
	/// Get current token
	fn peek(&self) -> Result<&TokenType> {
		self.get_token_at(0)
	}
	/// Get previous token
	fn prev(&self) -> Result<&TokenType> {
		self.get_token_at(-1)
	}
	/// Check if the current token is of a give type
	fn check(&self, token: &TokenType) -> Result<bool> {
		if self.is_at_end()? {
			return Ok(false);
		}
		Ok(self.peek()? == token)
	}
	/// Match the current token against a given list and advance the index only if there is a match
	fn match_type(&mut self, types: &'p [&TokenType]) -> Option<&TokenType> {
		for t in types {
			if self.check(t).unwrap() {
				self.advance().unwrap();
				return Some(t);
			}
		}
		None
	}
	fn get_op_token(&self) -> Result<&Operator> {
		match self.peek()? {
			TokenType::Operator(op) => Ok(op),
			other => bail!("This should be an operator '{:?}'", &other),
		}
	}

	fn expression(&mut self) -> Result<Expr> {
		self.equality()
	}
	fn equality(&mut self) -> Result<Expr> {
		let mut expr = self.comparison()?;

		while let Some(op) = self.match_type(&[
			&TokenType::Operator(Operator::EqEq),
			&TokenType::Operator(Operator::NotEq),
		]) {
			if let TokenType::Operator(op) = op {
				let op = op.clone();
				let right = self.comparison()?;

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
	fn comparison(&mut self) -> Result<Expr> {
		let mut expr = self.term()?;

		while let Some(op) = self.match_type(&[
			&TokenType::Operator(Operator::Gt),
			&TokenType::Operator(Operator::Gte),
			&TokenType::Operator(Operator::Lt),
			&TokenType::Operator(Operator::Lte),
		]) {
			if let TokenType::Operator(op) = op {
				let op = op.clone();
				let right = self.term()?;

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
	fn term(&mut self) -> Result<Expr> {
		let mut expr = self.factor()?;

		while let Some(op) = self.match_type(&[
			&TokenType::Operator(Operator::Sub),
			&TokenType::Operator(Operator::Add),
		]) {
			if let TokenType::Operator(op) = op {
				let op = op.clone();
				let right = self.factor()?;

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
	fn factor(&mut self) -> Result<Expr> {
		let mut expr = self.unary()?;

		while let Some(op) = self.match_type(&[
			&TokenType::Operator(Operator::Mul),
			&TokenType::Operator(Operator::Div),
		]) {
			if let TokenType::Operator(op) = op {
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
		if let Some(op) = self.match_type(&[
			&TokenType::Operator(Operator::Not),
			&TokenType::Operator(Operator::Sub),
		]) {
			if let TokenType::Operator(op) = op {
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
		if let TokenType::Literal(lit) = self.peek()? {
			let lit = match lit {
				token_type::Literal::String(v) => Ok(Expr::Literal {
					value: Literal::String(String::from_utf8(v.to_vec())?),
				}),
				token_type::Literal::Number(v) => Ok(Expr::Literal {
					value: Literal::Number(*v),
				}),
				token_type::Literal::Boolean(v) => Ok(Expr::Literal {
					value: Literal::Boolean(*v),
				}),
				token_type::Literal::Null => Ok(Expr::Literal {
					value: Literal::Null,
				}),
			};
			self.advance()?;

			lit
		} else if let Some(_token) =
			self.match_type(&[&TokenType::Punctuation(Punctuation::BracketOpen)])
		{
			let expr = self.expression()?;
			self.consume(&TokenType::Punctuation(Punctuation::BracketClose))?;

			Ok(Expr::Grouping {
				expr: Box::new(expr),
			})
		} else {
			unreachable!()
		}
	}
	fn consume(&mut self, until_token: &TokenType) -> Result<()> {
		if self.check(until_token)? {
			self.advance()?;
		}
		Ok(())
	}
}
