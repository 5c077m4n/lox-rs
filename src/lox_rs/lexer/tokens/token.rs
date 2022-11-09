use std::fmt;

use super::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<'t> {
	token_type: TokenType<'t>,
	line: usize,
	column: usize,
}

impl<'t> Token<'t> {
	pub fn new(token_type: TokenType<'t>, line: usize, column: usize) -> Self {
		Self {
			token_type,
			line,
			column,
		}
	}
	pub fn get(&self) -> &TokenType<'t> {
		&self.token_type
	}
}
impl<'t> fmt::Display for Token<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} @ {}:{}", self.token_type, self.line, self.column)
	}
}
