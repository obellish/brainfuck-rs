use std::{
	convert::Infallible,
	io::{stdin, stdout, Error as IoError, ErrorKind, Read, StdinLock, StdoutLock, Write},
	str::FromStr,
};

use thiserror::Error;

use super::{Cell, Instruction, Program, Tape};

#[derive(Debug, Clone)]
pub struct Interpreter<R: Read = StdinLock<'static>, W: Write = StdoutLock<'static>> {
	program: Program,
	memory: Tape,
	counter: usize,
	input: R,
	output: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
	pub const fn new(input: R, output: W) -> Self {
		Self {
			program: Program::new(),
			memory: Tape::new(),
			counter: 0,
			input,
			output,
		}
	}

	pub const fn program(&self) -> &Program {
		&self.program
	}

	pub fn program_mut(&mut self) -> &mut Program {
		&mut self.program
	}

	pub const fn memory(&self) -> &Tape {
		&self.memory
	}

	pub fn memory_mut(&mut self) -> &mut Tape {
		&mut self.memory
	}

	#[allow(unreachable_patterns, clippy::todo)]
	pub fn run(&mut self) -> Result<(), RuntimeError> {
		'program: loop {
			match *self.current_instruction() {
				Instruction::Increment(i) => self.memory += i,
				Instruction::Decrement(i) => self.memory -= i,
				Instruction::MoveLeft(i) => self.memory <<= i,
				Instruction::MoveRight(i) => self.memory >>= i,
				Instruction::Read => self.get_char()?,
				Instruction::Write => self.put_char()?,
				Instruction::JumpLeft => {
					if !matches!(self.current_cell().value(), 0) {
						let mut deep = 1;
						loop {
							if matches!(self.counter, 0) {
								break 'program;
							}
							self.counter -= 1;
							if matches!(self.current_instruction(), Instruction::JumpLeft) {
								deep += 1;
							}

							if matches!(self.current_instruction(), Instruction::JumpRight) {
								deep -= 1;
							}

							if matches!(deep, 0) {
								break;
							}
						}
					}
				}
				Instruction::JumpRight => {
					if matches!(self.current_cell().value(), 0) {
						let mut deep = 1;

						loop {
							if self.counter + 1 == self.program().len() {
								break 'program;
							}

							self.counter += 1;
							if matches!(self.current_instruction(), Instruction::JumpRight) {
								deep += 1;
							}

							if matches!(self.current_instruction(), Instruction::JumpLeft) {
								deep -= 1;
							}

							if matches!(deep, 0) {
								break;
							}
						}
					}
				}
				ref i => todo!("instruction not implemented: {i:?}"),
			}

			self.counter += 1;

			if self.program().len() == self.counter {
				break 'program;
			}
		}

		Ok(())
	}

	#[inline]
	pub fn current_instruction(&self) -> &Instruction {
		unsafe { self.program().get_unchecked(self.counter) }
	}

	#[inline]
	pub fn current_cell(&self) -> &Cell {
		self.memory.current_cell()
	}

	#[inline]
	pub fn current_cell_mut(&mut self) -> &mut Cell {
		self.memory.current_cell_mut()
	}

	fn get_char(&mut self) -> Result<(), RuntimeError> {
		let mut buf = [0];

		if let Err(error) = self.input.read_exact(&mut buf) {
			if !matches!(error.kind(), ErrorKind::UnexpectedEof) {
				return Err(error.into());
			}
		}

		self.current_cell_mut().set_value(buf[0]);

		Ok(())
	}

	fn put_char(&mut self) -> Result<(), RuntimeError> {
		let ch = self.current_cell().value();

		if ch.is_ascii() {
			self.output.write_all(&[ch])?;
		} else {
			write!(self.output, "\\0x{ch:x}")?;
		}

		self.output.flush()?;

		Ok(())
	}
}

impl Default for Interpreter {
	fn default() -> Self {
		Self::new(stdin().lock(), stdout().lock())
	}
}

impl FromStr for Interpreter {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let program = Program::from_str(s)?;

		Ok(Self {
			program,
			memory: Tape::new(),
			counter: 0,
			input: stdin().lock(),
			output: stdout().lock(),
		})
	}
}

#[derive(Debug, Error)]
pub enum RuntimeError {
	#[error(transparent)]
	Io(#[from] IoError),
}
