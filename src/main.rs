#![allow(unused)]

mod args;

use std::{
	fs,
	io::{empty, stdout, Cursor},
	path::PathBuf,
};

use anyhow::Result;
use brainfuck_rs::{Interpreter, Optimizer, Program};
use clap::Parser;

use self::args::Args;

#[allow(unused)]
pub fn main() -> Result<()> {
	let args = match Args::try_parse() {
		Ok(args) => args,
		Err(e) => {
			eprintln!("{e}");
			return Ok(());
		}
	};

	let stdout = std::io::stdout();

	let raw_data = fs::read_to_string(&args.input_path)?;

	let program = raw_data.parse::<Program>()?;

	let mut optimizer = Optimizer::new(program, args.verbose);

	let input = b"179424691\n";

	let mut interpreter = Interpreter::new(Cursor::new(input), stdout.lock());

	*interpreter.program_mut() = optimizer.optimize();

	interpreter.run();

	Ok(())
}
