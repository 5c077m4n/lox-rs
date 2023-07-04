#![allow(dead_code)]

use std::str::FromStr;

use anyhow::{bail, Result};
use tree_sitter::{Node, Range, TreeCursor};

#[derive(Default, Debug, PartialEq)]
pub enum Literal {
	Object,
	Array,
	String(String),
	Number(f64),
	Boolean(bool),
	Null,
	#[default]
	Undefined,
}

pub struct Interpreter<'i> {
	src: &'i str,
	tree: &'i TreeCursor<'i>,
}
impl<'i> Interpreter<'i> {
	pub fn new(src: &'i str, tree: &'i TreeCursor<'i>) -> Self {
		Self { src, tree }
	}

	fn node(&self) -> Node {
		self.tree.node()
	}
	fn fragment(&self) -> &str {
		let Range {
			start_byte,
			end_byte,
			..
		} = self.node().range();
		&self.src[start_byte..end_byte]
	}

	pub fn eval_expr(&self) -> Result<Literal> {
		let frag = self.fragment();

		match self.node().kind() {
			"number" => {
				let n = f64::from_str(frag)?;
				Ok(Literal::Number(n))
			}
			"string_framgment" => Ok(Literal::String(frag.to_owned())),
			"true" | "false" => {
				let b = bool::from_str(frag)?;
				Ok(Literal::Boolean(b))
			}
			"program" | "\"" => Ok(Literal::Undefined),
			other => {
				bail!("Type {} is not supported yet", other);
			}
		}
	}
}
