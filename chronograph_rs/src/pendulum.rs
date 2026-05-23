use super::moment::Moment;

pub trait Pendulum {
	type M: Moment;
	fn measurement(&mut self) -> Self::M;
}

pub struct InstantPenduram;

impl Pendulum for InstantPenduram {
	fn measurement(&mut self) -> <Self as Pendulum>::M {
		std::time::Instant::now()
	}
	type M = std::time::Instant;
}
