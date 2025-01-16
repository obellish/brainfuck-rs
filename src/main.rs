use std::{fs, path::PathBuf};

use anyhow::Result;
use brainfuck_rs::Interpreter;
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

	let mut interpreter = Interpreter::default();

	*interpreter.program_mut() = raw_data.parse()?;

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
