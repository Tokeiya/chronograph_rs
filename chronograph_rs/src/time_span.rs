use std::ops::AddAssign;
use std::time::Duration;

pub trait TimeSpan: AddAssign + Sized + Clone + Copy {
	fn zero() -> Self;
}

impl TimeSpan for Duration {
	fn zero() -> Self {
		Duration::default()
	}
}
