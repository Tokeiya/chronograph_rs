use crate::measurement::elapsed::Elapsed;
use crate::measurement::time_span::TimeSpan;
use std::ops::Index;

pub trait Memory<T: TimeSpan>: Index<usize, Output = Elapsed<T>> {
	type Iter<'a>: Iterator<Item = &'a Elapsed<T>>
	where
		Self: 'a,
		T: 'a;

	fn empty_like(&self) -> Self;
	fn iter(&self) -> Self::Iter<'_>;
	fn push(&mut self, value: Elapsed<T>);
	fn len(&self) -> usize;
	fn clear(&mut self);
}
