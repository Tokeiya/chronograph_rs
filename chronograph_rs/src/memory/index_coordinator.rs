pub(crate) struct IndexCoordinator<const N: usize>(usize);

impl<const N: usize> IndexCoordinator<N> {
	const CHECK: () = assert!(N.count_ones() == 1);
	const MASK: usize = N - 1;
	pub fn new() -> Self {
		_ = Self::CHECK;
		Self(0)
	}

	pub fn move_ahead(&mut self) {
		self.0 = self.0.wrapping_add(1) & Self::MASK;
	}

	pub fn move_back(&mut self) {
		self.0 = self.0.wrapping_sub(1) & Self::MASK;
	}

	pub fn convert(&self, idx: usize) -> usize {
		idx.wrapping_add(self.0) & Self::MASK
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	type Fixture = IndexCoordinator<8>;
	#[test]
	fn new() {
		let fixture = Fixture::new();
		assert_eq!(fixture.0, 0);
		assert_eq!(Fixture::MASK, 7);
	}
}
