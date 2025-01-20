use std::vec;

use super::{change::Change, Instruction, PeepholePass};

pub struct CombineInstPass;

impl PeepholePass for CombineInstPass {
	const SIZE: usize = 2;

	fn run_pass(&self, program: &[Instruction]) -> Change {
		assert_eq!(program.len(), Self::SIZE);
		match (program[0], program[1]) {
			(Instruction::Add(i1), Instruction::Add(i2)) => {
				if i1 == -i2 {
					Change::Remove
				} else {
					Change::Replace(vec![Instruction::Add(i1 + i2)])
				}
			}
			(Instruction::Move(i1), Instruction::Move(i2)) => {
				if i1 == -i2 {
					Change::Remove
				} else {
					Change::Replace(vec![Instruction::Move(i1 + i2)])
				}
			}
			_ => Change::Ignore,
		}
	}
}
