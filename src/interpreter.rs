use std::{
	convert::Infallible,
	io::{stdin, stdout, Error as IoError, ErrorKind, Read, StdinLock, StdoutLock, Write},
	str::FromStr,
};

use thiserror::Error;

use super::{Instruction, Program, Tape};
#[cfg(feature = "profiler")]
use crate::Profiler;

#[derive(Debug, Clone)]
pub struct Interpreter<R: Read = StdinLock<'static>, W: Write = StdoutLock<'static>> {
	program: Program,
	memory: Tape,
	counter: usize,
	input: R,
	output: W,
	#[cfg(feature = "profiler")]
	profiler: Profiler,
}

impl<R: Read, W: Write> Interpreter<R, W> {
	pub const fn new(input: R, output: W) -> Self {
		Self {
			program: Program::new(),
			memory: Tape::new(),
			counter: 0,
			input,
			output,
			#[cfg(feature = "profiler")]
			profiler: Profiler::new(),
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
			#[cfg(feature = "profiler")]
			{
				match self.current_instruction() {
					Instruction::Add(_) => *self.profiler.add_mut() += 1,
					Instruction::Move(_) => *self.profiler.mov_mut() += 1,
					Instruction::JumpLeft => *self.profiler.jl_mut() += 1,
					Instruction::JumpRight => *self.profiler.jr_mut() += 1,
					Instruction::Read => *self.profiler.inp_mut() += 1,
					Instruction::Write => *self.profiler.out_mut() += 1,
				}
			}

			match *self.current_instruction() {
				Instruction::Add(i) => *self.memory_mut() += i as u8,
				Instruction::Move(i) => {
					if i < 0 {
						*self.memory_mut() <<= i.unsigned_abs();
					} else {
						*self.memory_mut() >>= i.unsigned_abs();
					}
				}
				Instruction::Read => self.get_char()?,
				Instruction::Write => self.put_char()?,
				Instruction::JumpLeft => {
					if !matches!(self.current_cell(), 0) {
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
					if matches!(self.current_cell(), 0) {
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
	pub fn current_cell(&self) -> &u8 {
		self.memory.current_cell()
	}

	#[inline]
	pub fn current_cell_mut(&mut self) -> &mut u8 {
		self.memory.current_cell_mut()
	}

	fn get_char(&mut self) -> Result<(), RuntimeError> {
		let mut buf = [0];

		if let Err(error) = self.input.read_exact(&mut buf) {
			if !matches!(error.kind(), ErrorKind::UnexpectedEof) {
				return Err(error.into());
			}
		}

		*self.current_cell_mut() = buf[0];

		Ok(())
	}

	fn put_char(&mut self) -> Result<(), RuntimeError> {
		let ch = *self.current_cell();

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
			#[cfg(feature = "profiler")]
			profiler: Profiler::new(),
		})
	}
}

#[derive(Debug, Error)]
pub enum RuntimeError {
	#[error(transparent)]
	Io(#[from] IoError),
}
