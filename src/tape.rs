use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	ops::{AddAssign, Shl, ShlAssign, Shr, ShrAssign, SubAssign},
};

const TAPE_SIZE: usize = 1000;

#[derive(Clone)]
pub struct Tape {
	cells: [u8; TAPE_SIZE],
	pointer: usize,
}

impl Tape {
	#[must_use]
	pub const fn new() -> Self {
		// let cells = [0; TAPE_SIZE];

		// Self { cells, pointer: 0 }
		Self {
			cells: [0; TAPE_SIZE],
			pointer: 0,
		}
	}

	#[inline]
	#[must_use]
	pub fn current_cell(&self) -> &u8 {
		unsafe { self.cells.get_unchecked(self.pointer) }
	}

	#[inline]
	pub fn current_cell_mut(&mut self) -> &mut u8 {
		unsafe { self.cells.get_unchecked_mut(self.pointer) }
	}
}

impl AddAssign<u8> for Tape {
	#[inline]
	fn add_assign(&mut self, rhs: u8) {
		*self.current_cell_mut() = self.current_cell().wrapping_add(rhs);
	}
}

impl Debug for Tape {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let pretty_printing = f.alternate();
		let mut state = f.debug_list();

		for (i, cell) in self.cells.iter().enumerate() {
			if matches!(cell, 0)
				&& !pretty_printing
				&& self.cells[i..].iter().all(|c| matches!(c, 0))
			{
				return state.finish_non_exhaustive();
			}

			state.entry(&cell);
		}

		state.finish()
	}
}

impl Default for Tape {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Shl<usize> for Tape {
	type Output = Self;

	#[inline]
	fn shl(mut self, rhs: usize) -> Self::Output {
		self.shl_assign(rhs);
		self
	}
}

impl ShlAssign<usize> for Tape {
	#[inline]
	fn shl_assign(&mut self, rhs: usize) {
		self.pointer = if self.pointer.wrapping_sub(rhs) >= TAPE_SIZE {
			TAPE_SIZE - rhs
		} else {
			self.pointer - rhs
		};
	}
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr<usize> for Tape {
	type Output = Self;

	#[inline]
	fn shr(mut self, rhs: usize) -> Self::Output {
		self.shr_assign(rhs);
		self
	}
}

impl ShrAssign<usize> for Tape {
	#[inline]
	fn shr_assign(&mut self, rhs: usize) {
		self.pointer += rhs;

		if self.pointer >= TAPE_SIZE {
			self.pointer -= TAPE_SIZE;
		}
	}
}

impl SubAssign<u8> for Tape {
	#[inline]
	fn sub_assign(&mut self, rhs: u8) {
		*self.current_cell_mut() = self.current_cell().wrapping_sub(rhs);
	}
}
