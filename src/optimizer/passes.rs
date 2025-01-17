use std::{cmp::Ordering, vec};

use super::{change::Change, Instruction, PeepholePass};

pub struct CombineInstPass;

impl PeepholePass for CombineInstPass {
	const SIZE: usize = 2;

	fn run_pass(&self, program: &[Instruction]) -> Change {
		assert_eq!(program.len(), Self::SIZE);
		match (program[0], program[1]) {
			(Instruction::Increment(i1), Instruction::Increment(i2)) => {
				Change::Replace(vec![Instruction::Increment(i1.wrapping_add(i2))])
			}
			(Instruction::Decrement(i1), Instruction::Decrement(i2)) => {
				Change::Replace(vec![Instruction::Decrement(i1.wrapping_add(i2))])
			}
			(Instruction::Increment(i1), Instruction::Decrement(i2)) => match i1.cmp(&i2) {
				Ordering::Equal => Change::Remove,
				Ordering::Greater => Change::Replace(vec![Instruction::Increment(i1 - i2)]),
				Ordering::Less => Change::Replace(vec![Instruction::Decrement(i2 - i1)]),
			},
			(Instruction::Decrement(i1), Instruction::Increment(i2)) => match i1.cmp(&i2) {
				Ordering::Equal => Change::Remove,
				Ordering::Greater => Change::Replace(vec![Instruction::Decrement(i1 - i2)]),
				Ordering::Less => Change::Replace(vec![Instruction::Increment(i2 - i1)]),
			},
			(Instruction::MoveLeft(i1), Instruction::MoveLeft(i2)) => {
				Change::Replace(vec![Instruction::MoveLeft(i1.wrapping_add(i2))])
			}
			(Instruction::MoveRight(i1), Instruction::MoveRight(i2)) => {
				Change::Replace(vec![Instruction::MoveRight(i1.wrapping_add(i2))])
			}
			(Instruction::MoveRight(i1), Instruction::MoveLeft(i2)) => match i1.cmp(&i2) {
				Ordering::Equal => Change::Remove,
				Ordering::Greater => Change::Replace(vec![Instruction::MoveRight(i1 - i2)]),
				Ordering::Less => Change::Replace(vec![Instruction::MoveLeft(i2 - i1)]),
			},
			_ => Change::Ignore,
		}
	}
}
