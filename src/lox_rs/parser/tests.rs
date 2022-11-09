use anyhow::Result;

use super::{
	super::{
		ast::expr::{Expr, Literal, Stmt},
		lexer::{scanner::scan, tokens::token_type::Operator},
	},
	Parser,
};

#[test]
fn sanity() -> Result<()> {
	let input = b"null;";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(tree, &[Stmt::Expression(Expr::Literal(Literal::Null))]);

	Ok(())
}

#[test]
fn one_plus_one() -> Result<()> {
	let input = b"1 + 1;";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(
		tree,
		&[Stmt::Expression(Expr::Binary(
			Box::new(Expr::Literal(Literal::Number(1.))),
			Operator::Add,
			Box::new(Expr::Literal(Literal::Number(1.))),
		))]
	);

	Ok(())
}

#[test]
fn one_plus_one_mul_one() -> Result<()> {
	let input = b"1 + 1 * 1;";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(
		tree,
		&[Stmt::Expression(Expr::Binary(
			Box::new(Expr::Literal(Literal::Number(1.))),
			Operator::Add,
			Box::new(Expr::Binary(
				Box::new(Expr::Literal(Literal::Number(1.))),
				Operator::Mul,
				Box::new(Expr::Literal(Literal::Number(1.)))
			)),
		))]
	);

	Ok(())
}
