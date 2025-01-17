use super::Instruction;
use crate::InsertOrPush as _;

#[derive(Debug, Default, Clone)]
pub enum Change {
	#[default]
	Ignore,
	Remove,
	Replace(Vec<Instruction>),
}

impl Change {
	pub fn apply(self, ops: &mut Vec<Instruction>, i: usize, size: usize) -> (bool, usize) {
		match self {
			Self::Remove => {
				ops.drain(i..(i + size));

				(true, 0)
			}
			Self::Replace(instructions) => {
				for _ in 0..size {
					ops.remove(i);
				}

				for instr in instructions.into_iter().rev() {
					ops.insert_or_push(i, instr);
				}

				(true, 0)
			}
			Self::Ignore => (false, 0),
		}
	}
}
