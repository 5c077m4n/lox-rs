use anyhow::Result;

use super::{
	super::{
		ast::expr::{Expr, Literal},
		lexer::{scanner::scan, tokens::token_type::Operator},
	},
	Parser,
};

#[test]
fn sanity() -> Result<()> {
	let input = b"null";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(
		tree,
		Expr::Literal {
			value: Literal::Null
		}
	);

	Ok(())
}

#[test]
fn one_plus_one() -> Result<()> {
	let input = b"1 + 1";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(
		tree,
		Expr::Binary {
			left: Box::new(Expr::Literal {
				value: Literal::Number(1.)
			}),
			op: Operator::Add,
			right: Box::new(Expr::Literal {
				value: Literal::Number(1.)
			}),
		}
	);

	Ok(())
}

#[test]
fn one_plus_one_mul_one() -> Result<()> {
	let input = b"1 + 1 * 1";
	let input = scan(input);

	let mut parser = Parser::new(input);
	let (tree, errors) = parser.parse()?;

	assert!(errors.is_empty());
	assert_eq!(
		tree,
		Expr::Binary {
			left: Box::new(Expr::Literal {
				value: Literal::Number(1.)
			}),
			op: Operator::Add,
			right: Box::new(Expr::Binary {
				left: Box::new(Expr::Literal {
					value: Literal::Number(1.)
				}),
				op: Operator::Mul,
				right: Box::new(Expr::Literal {
					value: Literal::Number(1.)
				})
			}),
		}
	);

	Ok(())
}
