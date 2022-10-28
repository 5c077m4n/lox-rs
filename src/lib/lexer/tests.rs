use anyhow::Result;

use super::{
	super::lexer::tokens::token_type::{Keyword, TokenType},
	scanner::scan,
};
use crate::lib::lexer::tokens::token::Token;

#[test]
fn sanity() -> Result<()> {
	let input = b"null";
	let input: Vec<Token> = scan(input).collect();

	assert_eq!(
		input,
		vec![Token::new(TokenType::Keyword(Keyword::Null), "", 0, 0)]
	);
	Ok(())
}
