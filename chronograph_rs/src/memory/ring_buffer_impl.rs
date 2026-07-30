use super::lap_memory::Memory;
use crate::{Elapsed, TimeSpan};
use overwrite_ring_buffer::resizable::{Buffer, ResizableIndexCoordinator};
use overwrite_ring_buffer::{CircularBuffer, Iter};

impl<T: TimeSpan, C: ResizableIndexCoordinator> Memory<T> for Buffer<Elapsed<T>, C> {
	type Iter<'a>
		= Iter<'a, Elapsed<T>, C>
	where
		Self: 'a,
		T: 'a;

	fn empty_like(&self) -> Self {
		todo!()
	}

	fn iter(&self) -> Self::Iter<'_> {
		<Buffer<Elapsed<T>, C> as CircularBuffer<Elapsed<T>>>::iter(self)
	}

	fn push(&mut self, value: Elapsed<T>) {
		<Buffer<Elapsed<T>, C> as CircularBuffer<Elapsed<T>>>::enqueue(self, value)
	}

	fn len(&self) -> usize {
		<Buffer<Elapsed<T>, C> as CircularBuffer<Elapsed<T>>>::len(self)
	}

	fn clear(&mut self) {
		<Buffer<Elapsed<T>, C> as CircularBuffer<Elapsed<T>>>::clear(self)
	}
}
