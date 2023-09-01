use std::fmt;

use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token<'t> {
	token_type: TokenType<'t>,
	lexme: &'t str,
	// literal: _,
	line: usize,
	column: usize,
}

impl<'t> Token<'t> {
	pub fn new(token_type: TokenType<'t>, lexme: &'t str, line: usize, column: usize) -> Self {
		Self {
			token_type,
			lexme,
			line,
			column,
		}
	}
	pub fn to_str(&self) -> &str {
		self.lexme
	}
}
impl<'t> fmt::Display for Token<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:?} {} @ {}:{}",
			self.token_type, self.lexme, self.line, self.column
		)
	}
}
