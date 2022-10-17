use std::fmt;

use super::token_type::TokenType;

pub struct Token<'t> {
	token_type: TokenType,
	lexme: &'t str,
	// literal: _,
	line: usize,
	column: usize,
}

impl<'t> Token<'t> {
	pub fn new(token_type: TokenType, lexme: &'t str, line: usize, column: usize) -> Self {
		Self {
			token_type,
			lexme,
			line,
			column,
		}
	}
}
impl<'t> fmt::Display for Token<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!(
			"{:?} {} @ {}:{}",
			self.token_type, self.lexme, self.line, self.column
		))
	}
}
