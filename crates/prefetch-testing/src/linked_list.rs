use std::{intrinsics::*, mem, pin::Pin, task::Poll};

use futures::Stream;

use super::util::CoroutineIter;

#[derive(Debug, Default, Clone)]
pub enum List<T> {
	Cons(T, Box<Self>),
	#[default]
	Nil,
}

impl<T> List<T> {
	pub fn new(iter: impl IntoIterator<Item = T>) -> Self {
		let mut tail = Self::Nil;
		for item in iter {
			tail = Self::Cons(item, Box::new(tail));
		}

		tail
	}

	pub fn clear(&mut self) {
		if self.is_nil() {
			return;
		}

		let mut tmp = mem::take(self);
		loop {
			match tmp {
				Self::Nil => break,
				Self::Cons(_, next) => tmp = *next,
			}
		}
	}

	pub const fn is_nil(&self) -> bool {
		matches!(self, Self::Nil)
	}

	pub fn iter(&self) -> ListIterRef<'_, T> {
		self.into_iter()
	}

	pub fn into_generator(mut self) -> impl CoroutineIter<T> {
		#[coroutine]
		|| loop {
			match self {
				Self::Cons(t, tail) => {
					if tail.is_nil() {
						return t;
					}
					self = *tail;
					yield t;
				}
				Self::Nil => panic!("attempted to `resume` an empty generator"),
			}
		}
	}

	pub fn generator(&self) -> impl CoroutineIter<&T> {
		#[coroutine]
		move || {
			let mut gen = self;
			loop {
				match gen {
					Self::Cons(t, tail) => {
						if tail.is_nil() {
							return t;
						}
						gen = tail;
						yield t;
					}
					Self::Nil => panic!("attempted to `resume` an empty generator"),
				}
			}
		}
	}

	pub fn generator_prefetch(&self) -> impl CoroutineIter<&T> {
		#[coroutine]
		move || {
			let mut gen = self;
			loop {
				match gen {
					Self::Cons(t, tail) => {
						if tail.is_nil() {
							return t;
						}

						gen = tail;
						match &gen {
							Self::Cons(_, next) => unsafe { prefetch_read_data(&**next, 3) },
							Self::Nil => unsafe { unreachable() },
						}
						yield t;
					}
					Self::Nil => panic!("attempted to `resume` an empty generator"),
				}
			}
		}
	}
}

impl<T> IntoIterator for List<T> {
	type IntoIter = ListIter<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter {
		ListIter(self)
	}
}

impl<'a, T> IntoIterator for &'a List<T> {
	type IntoIter = ListIterRef<'a, T>;
	type Item = &'a T;

	fn into_iter(self) -> Self::IntoIter {
		ListIterRef(self)
	}
}

#[repr(transparent)]
pub struct ListIter<T>(List<T>);

impl<T> Iterator for ListIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let tmp = mem::take(&mut self.0);
		match tmp {
			List::Cons(t, next) => {
				self.0 = *next;
				Some(t)
			}
			List::Nil => None,
		}
	}
}

#[repr(transparent)]
pub struct ListIterRef<'a, T>(&'a List<T>);

impl<'a, T> Iterator for ListIterRef<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let mut tmp = &List::Nil;
		mem::swap(&mut tmp, &mut self.0);
		match tmp {
			List::Cons(t, next) => {
				self.0 = next;

				Some(t)
			}
			List::Nil => None,
		}
	}
}

#[repr(transparent)]
pub struct ListIterPrefetch<T>(List<T>);

impl<T> Iterator for ListIterPrefetch<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let tmp = mem::take(&mut self.0);
		match tmp {
			List::Cons(t, next) => {
				self.0 = *next;
				if let List::Cons(_, next) = &self.0 {
					unsafe { prefetch_read_data::<List<T>>(&**next, 3) }
				}
				Some(t)
			}
			List::Nil => None,
		}
	}
}

#[repr(transparent)]
pub struct ListIterPrefetchRef<'a, T>(&'a List<T>);

impl<'a, T> Iterator for ListIterPrefetchRef<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let mut tmp = &List::Nil;
		mem::swap(&mut tmp, &mut self.0);
		match tmp {
			List::Cons(t, next) => {
				self.0 = next;
				if let List::Cons(_, next) = &self.0 {
					unsafe {
						prefetch_read_data(&**next, 3);
					}
				}
				Some(t)
			}
			List::Nil => None,
		}
	}
}
