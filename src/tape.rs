use std::{
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	ops::{Add, AddAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign},
};

const TAPE_SIZE: usize = 1000;

#[derive(Clone)]
pub struct Tape {
	cells: [Cell; TAPE_SIZE],
	pointer: usize,
}

impl Tape {
	#[must_use]
	pub const fn new() -> Self {
		let mut cells = [Cell::Untouched; TAPE_SIZE];
		cells[0] = Cell::Value(0);

		Self { cells, pointer: 0 }
	}

	#[inline]
	#[must_use]
	pub fn current_cell(&self) -> &Cell {
		unsafe { self.cells.get_unchecked(self.pointer) }
	}

	#[inline]
	pub fn current_cell_mut(&mut self) -> &mut Cell {
		unsafe { self.cells.get_unchecked_mut(self.pointer) }
	}
}

impl AddAssign<u8> for Tape {
	#[inline]
	fn add_assign(&mut self, rhs: u8) {
		self.current_cell_mut().add_assign(rhs);
	}
}

impl Debug for Tape {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let pretty_printing = f.alternate();
		let mut state = f.debug_list();

		for (i, cell) in self.cells.iter().enumerate() {
			if matches!(cell, Cell::Untouched)
				&& !pretty_printing
				&& self.cells[i..].iter().all(|c| matches!(c, Cell::Untouched))
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
		for _ in 0..rhs {
			self.pointer = if matches!(self.pointer, 0) {
				TAPE_SIZE - 1
			} else {
				self.pointer - 1
			};

			self.current_cell_mut().touch();
		}
	}
}

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
		for _ in 0..rhs {
			self.pointer = if self.pointer == TAPE_SIZE - 1 {
				0
			} else {
				self.pointer + 1
			};

			self.current_cell_mut().touch();
		}
	}
}

impl SubAssign<u8> for Tape {
	#[inline]
	fn sub_assign(&mut self, rhs: u8) {
		self.current_cell_mut().sub_assign(rhs);
	}
}

#[derive(Default, Clone, Copy)]
pub enum Cell {
	#[default]
	Untouched,
	Value(u8),
}

impl Cell {
	#[inline]
	pub fn touch(&mut self) {
		if matches!(self, Self::Untouched) {
			*self = Self::Value(0);
		}
	}

	#[inline]
	#[must_use]
	pub const fn value(self) -> u8 {
		match self {
			Self::Untouched => 0,
			Self::Value(v) => v,
		}
	}

	#[inline]
	pub fn set_value(&mut self, value: u8) {
		*self = Self::Value(value);
	}
}

impl Add<u8> for Cell {
	type Output = Self;

	#[inline]
	fn add(self, rhs: u8) -> Self::Output {
		match self {
			Self::Untouched => Self::Value(rhs),
			Self::Value(lhs) => Self::Value(lhs.wrapping_add(rhs)),
		}
	}
}

impl AddAssign<u8> for Cell {
	#[inline]
	fn add_assign(&mut self, rhs: u8) {
		*self = *self + rhs;
	}
}

impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self, f)
	}
}

impl Display for Cell {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::Untouched => Display::fmt(&0, f),
			Self::Value(v) => Display::fmt(&v, f),
		}
	}
}

impl Sub<u8> for Cell {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: u8) -> Self::Output {
		match self {
			Self::Untouched => Self::Value(rhs),
			Self::Value(lhs) => Self::Value(lhs.wrapping_sub(rhs)),
		}
	}
}

impl SubAssign<u8> for Cell {
	#[inline]
	fn sub_assign(&mut self, rhs: u8) {
		*self = *self - rhs;
	}
}
