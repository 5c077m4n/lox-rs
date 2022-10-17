#![allow(dead_code)]
#![deny(clippy::all)]

mod lib;

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{self, Parser};

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
	let Args {
		filepath,
		eval,
		check_only: _,
	} = Args::parse();

	if let Some(filepath) = filepath {
		let _input = fs::read(&filepath)?;
	} else if let Some(input) = eval {
		let _input = input.as_str().as_bytes();
	}

	Ok(())
}
