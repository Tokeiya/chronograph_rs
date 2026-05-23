use super::duration_elapsed::Elapsed;
use super::error::{Error, Result};
use super::state::State;
use std::time::{Duration, Instant};

#[cfg(test)]
use super::test_now::now;

#[cfg(not(test))]
use super::now::now;

pub struct Chronograph {
	pivot: Instant,
	recent: Instant,
	accum: Duration,
	state: State,
	memory: Vec<Elapsed>,
}

impl Default for Chronograph {
	fn default() -> Self {
		Chronograph {
			pivot: now(),
			recent: now(),
			accum: Duration::new(0, 0),
			state: State::Ready,
			memory: Vec::new(),
		}
	}
}

impl Chronograph {
	pub fn state(&self) -> State {
		self.state
	}

	pub fn restart(&mut self) -> Duration {
		let ret = match self.state {
			State::Ready => Duration::new(0, 0),
			State::Running => {
				let current = now();
				self.accum + (current - self.pivot)
			}
			State::Stopped => self.accum,
		};

		self.pivot = now();
		self.accum = Duration::new(0, 0);
		self.state = State::Running;

		ret
	}

	pub fn start(&mut self) -> Result<Duration> {
		if self.state == State::Running {
			return Err(Error::AlreadyRunning);
		};

		self.pivot = now();
		self.recent = self.pivot;
		self.state = State::Running;
		Ok(self.accum)
	}

	pub fn stop(&mut self) -> Result<Duration> {
		if self.state == State::Stopped || self.state == State::Ready {
			return Err(Error::NotRunning);
		}
		self.accum += now() - self.pivot;
		self.state = State::Stopped;
		Ok(self.accum)
	}

	pub fn lap(&mut self) -> Result<(usize, &Elapsed)> {
		if self.state == State::Stopped {
			return Err(Error::NotRunning);
		}

		let current = now();
		let split = current - self.pivot;

		let lap = current - self.recent;
		self.recent = current;

		let elapsed = Elapsed::new(lap, split);
		self.memory.push(elapsed);
		Ok((self.memory.len(), self.memory.last().unwrap()))
	}

	pub fn reset(&mut self) -> Duration {
		let current = now();
		match self.state {
			State::Ready => Duration::ZERO,
			State::Running => {
				let ret = current - self.pivot;
				self.accum = Duration::new(0, 0);
				self.pivot = current;
				self.state = State::Ready;
				ret
			}
			State::Stopped => {
				let ret = self.accum;
				self.accum = Duration::new(0, 0);
				self.state = State::Ready;
				ret
			}
		}
	}

	pub fn clear(&mut self) -> (Vec<Elapsed>, Duration) {
		(std::mem::take(&mut self.memory), self.reset())
	}
	pub fn memory(&self) -> &[Elapsed] {
		&self.memory
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_now;

	fn assert(actual: &Elapsed, lap: Duration, split: Duration) {
		assert_eq!(actual.lap(), lap, "lap: {:?}", actual.lap());
		assert_eq!(actual.split(), split, "split: {:?}", actual.split());
	}

	#[test]
	fn state_check() {
		for _ in 0..10 {
			let recent = now();
			let current = now();
			assert_eq!(current - recent, Duration::new(1, 0));
		}
	}

	#[test]
	fn default() {
		let fixture = Chronograph::default();
		assert_eq!(fixture.state, State::Ready);
		assert_eq!(fixture.memory.len(), 0);
		assert_eq!(fixture.pivot, test_now::pivot());
		assert_eq!(fixture.accum, Duration::new(0, 0));
	}

	#[test]
	fn state() {
		let mut fixture = Chronograph::default();
		assert_eq!(fixture.state(), State::Ready);

		_ = fixture.start().unwrap();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.start().unwrap_err();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.stop();
		assert_eq!(fixture.state(), State::Stopped);

		_ = fixture.stop().unwrap_err();
		assert_eq!(fixture.state(), State::Stopped);

		_ = fixture.reset();
		assert_eq!(fixture.state(), State::Ready);

		_ = fixture.reset();
		assert_eq!(fixture.state(), State::Ready);

		_ = fixture.stop().unwrap_err();
		assert_eq!(fixture.state(), State::Ready);

		_ = fixture.start().unwrap();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.restart();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.restart();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.clear();
		assert_eq!(fixture.state(), State::Ready);
	}

	#[test]
	fn restart() {
		let mut fixture = Chronograph::default();
		_ = fixture.start().unwrap();
		let actual = fixture.restart();
		assert_eq!(actual, Duration::new(1, 0));
	}

	#[test]
	fn start() {
		let mut fixture = Chronograph::default();
		let actual = fixture.start().unwrap();
		assert_eq!(actual, Duration::new(0, 0));
		_ = fixture.start().unwrap_err();
	}

	#[test]
	fn stop() {
		let mut fixture = Chronograph::default();
		_ = fixture.start().unwrap();
		let actual = fixture.stop().unwrap();
		assert_eq!(actual, Duration::new(1, 0));
		_ = fixture.stop().unwrap_err();
	}

	#[test]
	fn lap_mem() {
		let mut fixture = Chronograph::default();
		_ = fixture.start().unwrap();
		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 1);
		assert(e, Duration::new(1, 0), Duration::new(1, 0));

		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 2);
		assert(e, Duration::new(1, 0), Duration::new(2, 0));

		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 3);
		assert(e, Duration::new(1, 0), Duration::new(3, 0));

		for (i, e) in fixture.memory().iter().enumerate() {
			assert(&e, Duration::new(1, 0), Duration::new((i + 1) as u64, 0))
		}
	}
}
