use crate::time_span::TimeSpan;
use std::ops::Sub;
use std::time::{Duration, Instant};

pub trait Moment: Sub<Output = Self::Span> + Ord + Copy + Sized {
	type Span: TimeSpan;
}

impl Moment for Instant {
	type Span = Duration;
}
