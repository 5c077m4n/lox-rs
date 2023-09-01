#![deny(clippy::all)]

mod lox_rs;

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::{arg, command, Parser};
use lox_rs::{
	ast::visitors::interp::Interperter,
	lexer::scanner::scan,
	parser::Parser as ASTParser,
};

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
		check_only,
		dump_ast,
	} = CLI::parse();

	let mut interp = Interperter::default();
	let input = if let Some(ref filepath) = filepath {
		fs::read(filepath)?
	} else if let Some(ref input) = eval {
		input.as_bytes().to_vec()
	} else {
		bail!("Could not find source code");
	};
	let input = scan(&input);

	let mut parser = ASTParser::new(input);
	let (tree, errors) = parser.parse()?;

	if !errors.is_empty() {
		bail!("{:?}", &errors);
	}

	for stmt in tree {
		if dump_ast {
			println!("{:#?}", &stmt);
		}
		if !check_only {
			if let Err(e) = stmt.interpret(&mut interp) {
				eprintln!("{e}");
			}
		}
	}

	Ok(())
}
