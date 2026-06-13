use super::lap_memory::Memory;
use crate::{Elapsed, TimeSpan};
use overwrite_ring_buffer::fixed::{Buffer, FixedIndexCoordinator};
use overwrite_ring_buffer::*;

impl<const N: usize, T: TimeSpan, C: FixedIndexCoordinator<N>> Memory<T>
	for Buffer<Elapsed<T>, C, N>
{
	type Iter<'a>
		= overwrite_ring_buffer::Iter<'a, Elapsed<T>, C>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		<Buffer<Elapsed<T>, C, N> as CircularBuffer<Elapsed<T>>>::iter(self)
	}

	fn push(&mut self, value: Elapsed<T>) {
		self.enqueue(value);
	}

	fn len(&self) -> usize {
		<Buffer<Elapsed<T>, C, N> as CircularBuffer<Elapsed<T>>>::len(self)
	}
}
