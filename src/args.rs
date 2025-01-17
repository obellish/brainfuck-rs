use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "None")]
pub struct Args {
	pub input_path: PathBuf,
	#[arg(short, long)]
	pub verbose: bool,
}
