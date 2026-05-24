use super::ring_buffer::RingBuffer;
use crate::{Elapsed, TimeSpan};
pub struct Iter<'a, T, const N: usize> {
	v: &'a RingBuffer<T, N>,
	idx: usize,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
	type Item = &'a Elapsed<T>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
