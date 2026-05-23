#![allow(clippy::disallowed_methods)]

use super::moment::Moment;
use std::time::Instant;

pub trait Pendulum {
	type Mmnt: Moment;
	fn measurement(&mut self) -> Self::Mmnt;
}

pub struct InstantPendulum;

impl Pendulum for InstantPendulum {
	type Mmnt = Instant;
	fn measurement(&mut self) -> <Self as Pendulum>::Mmnt {
		Instant::now()
	}
}
