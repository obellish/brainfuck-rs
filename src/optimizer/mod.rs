mod passes;

use self::passes::CombineInstPass;
use super::Program;

#[derive(Debug, Clone)]
pub struct Optimizer {
	program: Program,
}

impl Optimizer {
	#[must_use]
	pub const fn new(program: Program) -> Self {
		Self { program }
	}

	#[must_use]
	pub fn optimize(mut self) -> Program {
		let mut progress = false;

		progress |= CombineInstPass::new().run_pass(&mut self.program);

		if progress {
			self.optimize()
		} else {
			self.program
		}
	}
}

pub trait Pass {
	fn new() -> Self
	where
		Self: Sized;

	fn run_pass(self, program: &mut Program) -> bool;
}
