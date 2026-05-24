use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub struct Elapsed<T> {
	split: T,
	lap: T,
}

impl<T> Elapsed<T> {
	pub fn new(split: T, lap: T) -> Self {
		Self { split, lap }
	}
}

impl<T: Copy> Elapsed<T> {
	pub fn split(&self) -> T {
		self.split
	}

	pub fn lap(&self) -> T {
		self.lap
	}
}

impl<T: Debug> Debug for Elapsed<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(
			f,
			"Elapsed {{ split: {:?}, lap: {:?} }}",
			self.split, self.lap
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;
	type Target = Elapsed<Duration>;

	#[test]
	fn new() {
		for s in (10..20).map(|i| Duration::from_secs(i)) {
			for l in (0..10).map(|i| Duration::from_secs(i)) {
				let fixture = Elapsed::new(s, l);
				assert_eq!(fixture.split, s);
				assert_eq!(fixture.lap, l);
			}
		}
	}

	#[test]
	fn split() {
		for s in (10..20).map(|i| Duration::from_secs(i)) {
			for l in (0..10).map(|i| Duration::from_secs(i)) {
				let fixture = Elapsed::new(s, l);
				assert_eq!(fixture.split(), s);
			}
		}
	}

	#[test]
	fn lap() {
		for s in (10..20).map(|i| Duration::from_secs(i)) {
			for l in (0..10).map(|i| Duration::from_secs(i)) {
				let fixture = Elapsed::new(s, l);
				assert_eq!(fixture.lap(), l);
			}
		}
	}

	#[test]
	fn debug() {
		let fixture = Target::new(Duration::from_secs(10), Duration::from_secs(5));
		assert_eq!(format!("{:?}", fixture), "Elapsed { split: 10s, lap: 5s }");
	}
}
