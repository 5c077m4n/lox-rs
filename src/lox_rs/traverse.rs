use anyhow::{bail, Result};
use tree_sitter::{Node, TreeCursor};

pub fn traverse<F>(cursor: &mut TreeCursor, cb: &mut F) -> Result<()>
where
	F: FnMut(Node) -> Result<()>,
{
	let node = cursor.node();
	cb(node)?;

	if cursor.goto_first_child() {
		while {
			traverse(cursor, cb)?;
			cursor.goto_next_sibling()
		} {}

		if !cursor.goto_parent() {
			bail!("Could not go back to parent node")
		}
	}

	Ok(())
}
