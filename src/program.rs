use std::{
	convert::Infallible,
	fmt::{Debug, Formatter, Result as FmtResult},
	ops::{Deref, DerefMut},
	str::FromStr,
};

#[derive(Default, Clone)]
#[repr(transparent)]
pub struct Program {
	ops: Vec<Instruction>,
}

impl Program {
	#[must_use]
	pub const fn new() -> Self {
		Self { ops: Vec::new() }
	}
}

impl Debug for Program {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_list().entries(&**self).finish()
	}
}

impl Deref for Program {
	type Target = Vec<Instruction>;

	fn deref(&self) -> &Self::Target {
		&self.ops
	}
}

impl DerefMut for Program {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.ops
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

		Ok(Self { ops })
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
