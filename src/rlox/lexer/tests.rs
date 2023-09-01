use anyhow::Result;

use super::{
	super::lexer::tokens::{
		token::Token,
		token_type::{Literal, Operator, TokenType},
	},
	scanner::scan,
};

#[test]
fn sanity() -> Result<()> {
	let input = b"null";
	let input: Vec<Token> = scan(input).collect();

	assert_eq!(
		input,
		vec![Token::new(TokenType::Literal(Literal::Null), "", 0, 0)]
	);
	Ok(())
}

#[test]
fn one_plus_one() -> Result<()> {
	let input = b"1 + 1";
	let input: Vec<Token> = scan(input).collect();

	assert_eq!(
		input,
		vec![
			Token::new(TokenType::Literal(Literal::Number(1.)), "", 0, 0),
			Token::new(TokenType::Operator(Operator::Add), "", 0, 0),
			Token::new(TokenType::Literal(Literal::Number(1.)), "", 0, 0),
		]
	);
	Ok(())
}
