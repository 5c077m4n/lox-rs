use anyhow::Result;

use super::{
	super::{
		ast::expr::{Expr, Literal},
		lexer::scanner::scan,
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
