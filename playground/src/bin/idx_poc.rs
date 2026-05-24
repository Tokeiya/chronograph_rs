struct Poc<const N: u8>(u8);

impl<const N: u8> Poc<N> {
	const CHECK: () = assert!(N.count_ones() == 1);
	const MASK: u8 = N - 1;
	fn new() -> Self {
		_ = Self::CHECK;
		Self(0)
	}

	fn move_ahead(&mut self) {
		self.0 = self.0.wrapping_add(1) & Self::MASK;
	}

	fn convert(&self, idx: u8) -> u8 {
		self.0.wrapping_add(idx) & Self::MASK
	}
}

fn main() {
	let mut poc = Poc::<128>::new();

	for i in 0..127 {
		poc.move_ahead();
	}

	for i in 0..127 {
		println!("{i}:{}", poc.convert(i));
	}
}
