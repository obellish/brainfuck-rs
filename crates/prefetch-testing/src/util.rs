use std::{
	ops::{Coroutine, CoroutineState},
	pin::Pin,
	time::Instant,
};

use futures::{Future, Stream};

pub trait CoroutineIter<T> = Coroutine<Yield = T, Return = T>;
