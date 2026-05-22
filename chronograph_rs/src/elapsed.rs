use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Elapsed {
	split: Duration,
	lap: Duration,
}

impl Elapsed {
	pub fn new(lap: Duration, split: Duration) -> Self {
		debug_assert!(lap <= split, "lap must be less than or equal to split");
		Self { lap, split }
	}

	pub fn lap(&self) -> Duration {
		self.lap
	}

	pub fn split(&self) -> Duration {
		self.split
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::ops::Sub;

	#[test]
	fn new() {
		let fixture = Elapsed::new(Duration::from_secs(1), Duration::from_secs(10));
		assert_eq!(fixture.lap, Duration::from_secs(1));
		assert_eq!(fixture.split, Duration::from_secs(10));

		let fixture = Elapsed::new(Duration::from_secs(0), Duration::from_secs(0));
		assert_eq!(fixture.lap, Duration::from_secs(0));
		assert_eq!(fixture.split, Duration::from_secs(0));
	}

	#[test]
	#[should_panic]
	fn invalid_recent_new() {
		_ = Elapsed::new(Duration::from_secs(10), Duration::from_secs(1));
	}

	#[test]
	fn lap() {
		let fixture = Elapsed::new(Duration::from_secs(1), Duration::from_secs(10));
		assert_eq!(fixture.lap(), Duration::from_secs(1));
	}

	#[test]
	fn split() {
		let fixture = Elapsed::new(Duration::from_secs(1), Duration::from_secs(10));
		assert_eq!(fixture.split(), Duration::from_secs(10));
	}
}
