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

	if let Some(filepath) = filepath {
		let input = fs::read(filepath)?;
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
	} else if let Some(input) = eval {
		let input = input.as_str().as_bytes();
		let input = scan(input);

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
	}

	Ok(())
}
