#![allow(dead_code)]
#![deny(clippy::all)]

mod lox_rs;

use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::{self, Parser};
use lox_rs::{lexer::scanner::scan, parser::Parser as ASTParser};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	pub filepath: Option<PathBuf>,
	#[clap(short, long)]
	pub eval: Option<String>,
	#[clap(short, long)]
	pub check_only: bool,
	#[clap(long)]
	pub dump_ast: bool,
}

fn main() -> Result<()> {
	env_logger::init();

	let Args {
		filepath,
		eval,
		check_only,
		dump_ast,
	} = Args::parse();

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
				if let Err(e) = stmt.interpret() {
					eprintln!("{}", e);
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
				if let Err(e) = stmt.interpret() {
					eprintln!("{}", e);
				}
			}
		}
	}

	Ok(())
}
