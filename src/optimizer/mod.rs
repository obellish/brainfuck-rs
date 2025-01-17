mod change;
mod passes;

use self::{change::Change, passes::CombineInstPass};
use super::{Instruction, Program};

#[derive(Debug, Clone)]
pub struct Optimizer {
	program: Program,
	verbose: bool,
}

impl Optimizer {
	#[must_use]
	pub const fn new(program: Program, verbose: bool) -> Self {
		Self { program, verbose }
	}

	#[must_use]
	pub fn optimize(self) -> Program {
		self.optimize_inner(1)
	}

	fn optimize_inner(mut self, iteration: usize) -> Program {
		let starting_instruction_count = self.program.len();

		let mut progress = false;

		progress |= self.run_pass(CombineInstPass);

		if progress {
			if self.verbose {
				eprintln!("Optimization iteration {iteration}: {starting_instruction_count} -> {}, continuing...", self.program.len());
			}
			self.optimize_inner(iteration + 1)
		} else {
			if self.verbose {
				eprintln!("Optimization iteration {iteration}: none applied, finished with {} instructions", self.program.len());
			}
			self.program.into_optimized()
		}
	}

	fn run_pass<P>(&mut self, pass: P) -> bool
	where
		P: Pass,
	{
		pass.run_pass(&mut self.program)
	}
}

pub trait Pass {
	fn run_pass(&self, program: &mut Program) -> bool;
}

pub trait PeepholePass {
	const SIZE: usize;

	fn run_pass(&self, window: &[Instruction]) -> Change;
}

impl<P> Pass for P
where
	P: PeepholePass,
{
	fn run_pass(&self, program: &mut Program) -> bool {
		let mut i = 0;

		let mut progress = false;

		while program.len() >= P::SIZE && i < program.len() - (P::SIZE - 1) {
			let window = &program[i..(P::SIZE + i)];

			let change = P::run_pass(self, window);

			let (changed, removed) = change.apply(program.as_raw(), i, P::SIZE);
			i -= removed;

			if changed {
				progress = true;
			} else {
				i += 1;
			}
		}

		progress
	}
}
