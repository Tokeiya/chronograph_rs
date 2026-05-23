use super::error::{Error, Result};
use super::pendulum::Pendulum;
use super::time_span::TimeSpan;
use crate::elapsed::Elapsed;
use crate::moment::Moment;
use crate::state::State;

pub struct Chronograph<M, T, P> {
	pendulum: P,
	pivot: M,
	recent: M,
	accum: T,
	state: State,
	memory: Vec<Elapsed<T>>,
}

impl<M, T, P> Chronograph<M, T, P>
where
	T: TimeSpan,
	M: Moment<Span = T>,
	P: Pendulum<Mmnt = M>,
{
	pub fn new(mut pendulum: P) -> Self {
		let tmp = pendulum.measurement();

		Self {
			pendulum,
			pivot: tmp,
			recent: tmp,
			accum: T::default(),
			state: State::Ready,
			memory: Vec::new(),
		}
	}

	pub fn state(&self) -> State {
		self.state
	}

	pub fn restart(&mut self) -> T {
		todo!()
	}

	pub fn start(&mut self) -> Result<T> {
		todo!()
	}

	pub fn stop(&mut self) -> Result<T> {
		todo!()
	}

	pub fn lap(&mut self) -> Result<(usize, Elapsed<T>)> {
		todo!()
	}

	pub fn reset(&mut self) -> T {
		todo!()
	}

	pub fn clear(&mut self) -> (Vec<Elapsed<T>>, T) {
		todo!()
	}

	pub fn memory(&self) -> &[Elapsed<T>] {
		&self.memory
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::mock;
	use mockall::predicate::*;
	use std::fmt::Debug;

	impl TimeSpan for usize {}
	impl Moment for usize {
		type Span = usize;
	}

	mock! {
		Dummy{}

		impl Pendulum for Dummy {
		type Mmnt = usize;
		fn measurement(&mut self) -> usize;
		}

	}

	#[test]
	fn new() {
		let mut mock = MockDummy::new();
		mock.expect_measurement().times(1).returning(|| 42);

		let fixture = Chronograph::new(mock);
		assert_eq!(fixture.state, State::Ready);
		assert_eq!(fixture.pivot, 42);
		assert_eq!(fixture.recent, 42);
		assert_eq!(fixture.accum, 0);
		assert_eq!(fixture.memory.len(), 0);
	}

	#[test]
	fn state() {
		let mut mock = MockDummy::new();
		mock.expect_measurement().return_const(42usize);
		let mut fixture = Chronograph::new(mock);
		assert_eq!(fixture.state(), State::Ready);

		fixture.start().unwrap();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.start().unwrap_err();
		assert_eq!(fixture.state(), State::Running);

		fixture.lap().unwrap();
		assert_eq!(fixture.state(), State::Running);

		fixture.stop().unwrap();
		assert_eq!(fixture.state(), State::Stopped);

		_ = fixture.stop().unwrap_err();
		assert_eq!(fixture.state(), State::Stopped);

		_ = fixture.restart();
		assert_eq!(fixture.state(), State::Running);

		_ = fixture.reset();
		assert_eq!(fixture.state(), State::Ready);

		_ = fixture.clear();
		assert_eq!(fixture.state(), State::Ready);
	}

	fn gen_mock() -> MockDummy {
		let mut mock = MockDummy::new();
		let mut cnt = 0usize;

		mock.expect_measurement().returning(move || {
			let ret = cnt;
			cnt += 1;
			ret
		});

		mock
	}

	#[test]
	fn restart() {
		let mut fixture = Chronograph::new(gen_mock());
		assert_eq!(fixture.restart(), 0);
		assert_eq!(fixture.state, State::Running);
		assert_eq!(fixture.restart(), 1);
		assert_eq!(fixture.state, State::Running);
	}

	#[test]
	fn start() {
		let mut fixture = Chronograph::new(gen_mock());
		assert!(matches!(fixture.start(), Ok(x) if x == 0));
		assert_eq!(fixture.state, State::Running);
		assert!(matches!(fixture.start(), Err(Error::AlreadyRunning)));
		assert_eq!(fixture.state, State::Running);

		_ = fixture.stop().unwrap();
		assert!(matches!(fixture.start(), Ok(x) if x == 1));
	}

	#[test]
	fn stop() {
		let mut fixture = Chronograph::new(gen_mock());
		_ = fixture.start().unwrap();
		assert!(matches!(fixture.stop(), Ok(x) if x == 1));
		assert_eq!(fixture.state, State::Stopped);
		assert!(matches!(fixture.stop(), Err(Error::NotRunning)));
		assert_eq!(fixture.state, State::Stopped);

		_ = fixture.start().unwrap();
		assert!(matches!(fixture.stop(), Ok(x) if x == 2));
	}

	fn assert_elapsed<T: TimeSpan + PartialEq + Debug>(elapsed: &Elapsed<T>, split: T, lap: T) {
		assert_eq!(elapsed.split(), split);
		assert_eq!(elapsed.lap(), lap);
	}

	#[test]
	fn lap() {
		let mut fixture = Chronograph::new(gen_mock());
		_ = fixture.start().unwrap();

		for i in 1..=10 {
			let (cnt, elapsed) = fixture.lap().unwrap();
			assert_elapsed(&elapsed, i, 1);
			assert_eq!(cnt, i);
			assert_eq!(fixture.memory.len(), i);
			assert_elapsed(&fixture.memory[i - 1], i, 1)
		}
	}

	#[test]
	fn reset() {
		let mut fixture = Chronograph::new(gen_mock());
		_ = fixture.start().unwrap();
		assert_eq!(fixture.reset(), 1);
		assert_eq!(fixture.reset(), 0);
	}

	#[test]
	fn clear() {
		let mut fixture = Chronograph::new(gen_mock());
		_ = fixture.start().unwrap();

		for _ in 0..10 {
			_ = fixture.lap().unwrap();
		}

		let (mem, ttl) = fixture.clear();
		assert_eq!(mem.len(), 10);
		assert_eq!(ttl, 10);

		for (idx, ela) in mem.iter().enumerate() {
			assert_elapsed(ela, idx + 1, 1);
		}

		assert_eq!(fixture.memory.len(), 0);
	}

	#[test]
	fn memory() {
		let mut fixture = Chronograph::new(gen_mock());
		_ = fixture.start().unwrap();

		for i in 0..10 {
			assert_eq!(fixture.memory.len(), i);
			let _ = fixture.lap().unwrap();
			assert_eq!(fixture.memory.len(), i + 1);
			assert_elapsed(&fixture.memory()[i], i + 1, 1);
		}
	}

	#[test]
	fn stop_start_consistency() {
		let mut mock = MockDummy::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_measurement()
			.in_sequence(&mut seq)
			.return_const(1usize);
		mock.expect_measurement()
			.in_sequence(&mut seq)
			.return_const(5usize);
		mock.expect_measurement()
			.in_sequence(&mut seq)
			.return_const(42usize);
		mock.expect_measurement()
			.in_sequence(&mut seq)
			.return_const(45usize);

		let mut fixture = Chronograph::new(mock);
		_ = fixture.start().unwrap();
		assert_eq!(fixture.stop().unwrap(), 4);
		assert_eq!(fixture.start().unwrap(), 4);
		assert_eq!(fixture.stop().unwrap(), 7);
	}
}
