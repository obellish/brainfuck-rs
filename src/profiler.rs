#[derive(Debug, Clone)]
pub struct Profiler {
	add: u64,
	mov: u64,
	jr: u64,
	jl: u64,
	inp: u64,
	out: u64,
}

impl Profiler {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			add: 0,
			mov: 0,
			jr: 0,
			jl: 0,
			inp: 0,
			out: 0,
		}
	}

	#[must_use]
	pub const fn add(&self) -> &u64 {
		&self.add
	}

	pub fn add_mut(&mut self) -> &mut u64 {
		&mut self.add
	}

	#[must_use]
	pub const fn mov(&self) -> &u64 {
		&self.mov
	}

	pub fn mov_mut(&mut self) -> &mut u64 {
		&mut self.mov
	}

	#[must_use]
	pub const fn jr(&self) -> &u64 {
		&self.jr
	}

	pub fn jr_mut(&mut self) -> &mut u64 {
		&mut self.jr
	}

	#[must_use]
	pub const fn jl(&self) -> &u64 {
		&self.jl
	}

	pub fn jl_mut(&mut self) -> &mut u64 {
		&mut self.jl
	}

	#[must_use]
	pub const fn inp(&self) -> &u64 {
		&self.inp
	}

	pub fn inp_mut(&mut self) -> &mut u64 {
		&mut self.inp
	}

	#[must_use]
	pub const fn out(&self) -> &u64 {
		&self.out
	}

	pub fn out_mut(&mut self) -> &mut u64 {
		&mut self.out
	}
}

impl Default for Profiler {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for Profiler {
	fn drop(&mut self) {
		println!("profile:");
		println!(" +: {}", self.add());
		println!(" >: {}", self.mov());
		println!(" [: {}", self.jr());
		println!(" ]: {}", self.jl());
		println!(" .: {}", self.out());
		println!(" ,: {}", self.inp());
	}
}
