use std::iter::{self, Peekable};

use log::error;

use super::{
	detector::detect,
	tokens::{
		token::Token,
		token_type::{Punctuation, TokenType},
	},
};

pub fn scan(mut input: &[u8]) -> Box<Peekable<impl Iterator<Item = Token<'_>>>> {
	Box::new(
		iter::from_fn({
			move || match detect(input) {
				Ok((tail, token_type)) => {
					input = tail;

					if token_type != TokenType::EndOfFile {
						Some(Token::new(token_type, 0, 0))
					} else {
						None
					}
				}
				Err(error) => {
					error!("{:#?}", &error);
					None
				}
			}
		})
		.filter_map(|t| match *t.get() {
			TokenType::Punctuation(Punctuation::Space) => None,
			TokenType::Punctuation(Punctuation::Tab) => None,
			TokenType::EndOfLine => None,
			_ => Some(t),
		})
		.peekable(),
	)
}
