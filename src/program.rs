use std::{
	convert::Infallible,
	fmt::{Debug, Formatter, Result as FmtResult},
	ops::{Deref, DerefMut},
	str::FromStr,
};

#[derive(Clone)]
pub enum Program {
	Raw(Vec<Instruction>),
	Optimized(Box<[Instruction]>),
}

impl Program {
	#[must_use]
	pub const fn new() -> Self {
		Self::Raw(Vec::new())
	}

	pub fn as_raw(&mut self) -> &mut Vec<Instruction> {
		match self {
			Self::Raw(ops) => ops,
			Self::Optimized(ops) => {
				*self = Self::Raw(ops.to_vec());

				match self {
					Self::Raw(ops) => ops,
					Self::Optimized(_) => unreachable!(),
				}
			}
		}
	}

	#[must_use]
	pub fn into_optimized(self) -> Self {
		if let Self::Raw(v) = self {
			Self::Optimized(v.into_boxed_slice())
		} else {
			self
		}
	}
}

impl Debug for Program {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_list().entries(&**self).finish()
	}
}

impl Default for Program {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for Program {
	type Target = [Instruction];

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Raw(ops) => ops,
			Self::Optimized(ops) => ops,
		}
	}
}

impl DerefMut for Program {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Raw(ops) => ops,
			Self::Optimized(ops) => ops,
		}
	}
}

impl FromStr for Program {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ops = Vec::new();
		for b in s.bytes() {
			let inst = match b {
				b'+' => Instruction::Increment(1),
				b'-' => Instruction::Decrement(1),
				b'.' => Instruction::Write,
				b',' => Instruction::Read,
				b'>' => Instruction::MoveRight(1),
				b'<' => Instruction::MoveLeft(1),
				b'[' => Instruction::JumpRight,
				b']' => Instruction::JumpLeft,
				_ => continue,
			};

			ops.push(inst);
		}

		Ok(Self::Raw(ops))
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	MoveRight(usize),
	MoveLeft(usize),
	Increment(u8),
	Decrement(u8),
	Write,
	Read,
	JumpRight,
	JumpLeft,
}
