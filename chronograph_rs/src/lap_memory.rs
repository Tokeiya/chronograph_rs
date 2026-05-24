use super::elapsed::Elapsed;
use super::time_span::TimeSpan;
use std::ops::Index;
use std::process::Output;
use std::time::Duration;

pub trait Memory<T: TimeSpan>: Index<usize, Output = Elapsed<T>> {
	type Iter<'a>: Iterator<Item = &'a Elapsed<T>>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_>;

	fn push(&mut self, value: Elapsed<T>);
	fn len(&self) -> usize;
}
