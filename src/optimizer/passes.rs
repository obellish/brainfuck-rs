use super::{Pass, Program};

pub struct CombineInstPass;

impl Pass for CombineInstPass {
	fn new() -> Self
	where
		Self: Sized,
	{
		Self
	}

	fn run_pass(self, _program: &mut Program) -> bool {
		false
	}
}
