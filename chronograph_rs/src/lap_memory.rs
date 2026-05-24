use super::elapsed::Elapsed;
use super::time_span::TimeSpan;
use std::ops::Index;
use std::time::Duration;

pub trait Memory<T: TimeSpan>: Index<usize> {
	type Iter<'a>: Iterator<Item = &'a Elapsed<T>>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_>;

	fn input(&mut self, value: Elapsed<T>);
}

impl<T: TimeSpan> Memory<T> for Vec<Elapsed<T>> {
	type Iter<'a>
		= std::slice::Iter<'a, Elapsed<T>>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}

	fn input(&mut self, value: Elapsed<T>) {
		self.push(value);
	}
}
