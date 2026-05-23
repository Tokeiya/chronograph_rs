use crate::time_span::TimeSpan;
use std::ops::Sub;

pub trait Moment<T: TimeSpan>: Sub<Output = T> + Ord + Sized {}
