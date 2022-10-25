use std::iter::{self, Peekable};

use log::{debug, error};

use super::{detector::detect, tokens::token::Token};
use crate::lib::lexer::tokens::token_type::TokenType;

pub fn scan(mut input: &[u8]) -> Box<Peekable<impl Iterator<Item = Token<'_>>>> {
	Box::new(
		iter::from_fn({
			move || match detect(input) {
				Ok((tail, token_type)) => {
					input = tail;
					debug!("{:#?}", &token_type);

					if token_type != TokenType::Empty {
						Some(Token::new(token_type, "", 0, 0))
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
		.peekable(),
	)
}
