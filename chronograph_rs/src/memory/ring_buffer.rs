use super::lap_memory::Memory;
use crate::measurement::{elapsed::Elapsed, time_span::TimeSpan};
use std::ops::Index;

pub struct RingBuffer<T, const N: usize> {
	storage: [T; N],
	len: usize,
	head: usize,
}

const fn check(n: usize) {
	assert!(n.count_ones() == 1);
}

impl<T, const N: usize> RingBuffer<T, N> {
	const CHECK: () = assert!(N.count_ones() == 1);
	const MASK: usize = N - 1;

	pub fn new() -> Self {
		todo!()
	}

	fn inclement_head(&mut self) {
		todo!()
	}

	fn virtual_to_real(idx: usize) -> usize {
		todo!()
	}

	fn real_to_virtual(idx: usize) -> usize {
		todo!()
	}
}

impl<T, const N: usize> Index<usize> for RingBuffer<T, N> {
	type Output = Elapsed<T>;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
	fn default() -> Self {
		todo!()
	}
}

impl<T: TimeSpan, const N: usize> Memory<T> for RingBuffer<T, N> {
	type Iter<'a>
		= super::ring_buffer_iter::Iter<'a, T, N>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		todo!()
	}

	fn push(&mut self, value: Elapsed<T>) {
		todo!()
	}

	fn len(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	type Fixture = RingBuffer<u64, 8>;

	#[test]
	fn new() {
		let fixture = Fixture::new();
		assert_eq!(fixture.len, 0);
	}

	#[test]
	fn default() {
		let fixture = Fixture::default();
		assert_eq!(fixture.len, 0);
	}
}
