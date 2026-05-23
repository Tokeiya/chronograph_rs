use super::moment::Moment;

pub trait Pendulum {
	type M: Moment;
	fn measurement(&mut self) -> Self::M;
}

pub struct InstantPendulum;

impl Pendulum for InstantPendulum {
	fn measurement(&mut self) -> <Self as Pendulum>::M {
		std::time::Instant::now()
	}
	type M = std::time::Instant;
}
