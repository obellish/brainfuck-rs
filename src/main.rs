#![allow(unused)]

use std::{
	fs,
	io::{empty, stdout, Cursor},
	path::PathBuf,
};

use anyhow::Result;
use brainfuck_rs::{Interpreter, Optimizer, Program};
use clap::Parser;

#[allow(unused)]
pub fn main() -> Result<()> {
	let args = match Args::try_parse() {
		Ok(args) => args,
		Err(e) => {
			eprintln!("{e}");
			return Ok(());
		}
	};

	let raw_data = fs::read_to_string(&args.input_path)?;

	let program = raw_data.parse::<Program>()?;

	let mut optimizer = Optimizer::new(program);

	let input = b"179424691\n";

	let mut interpreter = Interpreter::new(Cursor::new(input), stdout());

	*interpreter.program_mut() = optimizer.optimize();

	println!("{:?}", interpreter.memory());

	interpreter.run();

	println!("{:?}", interpreter.memory());

	Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "None")]
struct Args {
	pub input_path: PathBuf,
}
