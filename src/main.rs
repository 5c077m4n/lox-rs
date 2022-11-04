#![allow(dead_code)]
#![deny(clippy::all)]

mod rlox;

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::{self, Parser};
use rlox::{lexer::scanner::scan, parser::Parser as ASTParser};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	pub filepath: Option<PathBuf>,

	#[clap(short, long)]
	pub eval: Option<String>,

	#[clap(short, long)]
	pub check_only: bool,
}

fn main() -> Result<()> {
	env_logger::init();

	let Args {
		filepath,
		eval,
		check_only: _,
	} = Args::parse();

	if let Some(filepath) = filepath {
		let input = fs::read(&filepath)?;
		let input = scan(&input);

		let mut parser = ASTParser::new(input);
		let (tree, errors) = parser.parse()?;

		if !errors.is_empty() {
			bail!("{:?}", &errors);
		}

		tree.dump();
	} else if let Some(input) = eval {
		let input = input.as_str().as_bytes();
		let input = scan(input);

		let mut parser = ASTParser::new(input);
		let (tree, errors) = parser.parse()?;

		if !errors.is_empty() {
			bail!("{:?}", &errors);
		}

		tree.dump();
	}

	Ok(())
}
