#![deny(clippy::all)]

mod lox_rs;

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::{arg, command, Parser};
use log::{error, info};
use tree_sitter::{Parser as TreeParser, Range};
use tree_sitter_typescript::language_typescript;

use crate::lox_rs::traverse;

#[derive(Parser, Debug)]
#[command(about, version, author)]
pub struct CLI {
	pub filepath: Option<PathBuf>,
	#[arg(short, long)]
	pub eval: Option<String>,
	#[arg(short, long)]
	pub check_only: bool,
	#[arg(long)]
	pub dump_ast: bool,
}

fn main() -> Result<()> {
	env_logger::init();

	let CLI {
		filepath,
		eval,
		check_only: _,
		dump_ast: _,
	} = CLI::parse();

	let lang = language_typescript();
	let mut parser = TreeParser::new();
	parser.set_language(lang)?;

	let input = if let Some(ref filepath) = filepath {
		fs::read(filepath)?
	} else if let Some(ref input) = eval {
		input.as_bytes().to_owned()
	} else {
		bail!("No source code found")
	};
	let input = std::str::from_utf8(&input)?;

	let Some(tree) = parser.parse(input, None) else {
		bail!("The input could not be parsed");
	};
	println!("{:#?}", tree.root_node().to_sexp());

	let mut cursor = tree.walk();
	traverse(&mut cursor, &mut |n| {
		let Range {
			start_byte,
			end_byte,
			..
		} = n.range();
		if n.is_error() {
			error!("{:?}, {:?}", n, &input[start_byte..end_byte]);
		} else {
			info!("{:?}, {:?}", n, &input[start_byte..end_byte]);
		}
		Ok(())
	})?;

	Ok(())
}
