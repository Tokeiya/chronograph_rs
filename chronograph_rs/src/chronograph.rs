use super::error::{Error, Result};
use super::measurement::pendulum::Pendulum;
use super::measurement::time_span::TimeSpan;
use super::memory::lap_memory::Memory;
use crate::measurement::elapsed::Elapsed;
use crate::measurement::moment::Moment;
use crate::state::State;

type MomentOf<P> = <P as Pendulum>::Mmnt;
type SpanOf<P> = <<P as Pendulum>::Mmnt as Moment>::Span;

pub struct Chronograph<P: Pendulum, M: Memory<SpanOf<P>>> {
	pendulum: P,
	pivot: MomentOf<P>,
	recent: MomentOf<P>,
	accum: SpanOf<P>,
	state: State,
	memory: M,
}

impl<P, M> Chronograph<P, M>
where
	P: Pendulum,
	M: Memory<SpanOf<P>>,
{
	pub fn new(mut pendulum: P, memory: M) -> Self {
		let tmp = pendulum.measurement();

		Self {
			pendulum,
			pivot: tmp,
			recent: tmp,
			accum: SpanOf::<P>::zero(),
			state: State::Ready,
			memory,
		}
	}

	pub fn state(&self) -> State {
		self.state
	}

	fn calc_accum(&mut self, current: MomentOf<P>) -> SpanOf<P> {
		match self.state {
			State::Ready => SpanOf::<P>::zero(),
			State::Running => {
				self.accum += current - self.pivot;
				self.accum
			}
			State::Stopped => self.accum,
		}
	}

	pub fn restart(&mut self) -> SpanOf<P> {
		let current = self.pendulum.measurement();
		let ret = self.calc_accum(current);

		let current = self.pendulum.measurement();
		self.state = State::Running;
		self.pivot = current;
		self.recent = current;
		self.accum = SpanOf::<P>::zero();

		ret
	}

	pub fn start(&mut self) -> Result<SpanOf<P>> {
		let current = self.pendulum.measurement();
		if self.state == State::Running {
			Err(Error::AlreadyRunning)
		} else {
			self.pivot = current;
			self.recent = current;
			self.state = State::Running;
			Ok(self.accum)
		}
	}

	pub fn stop(&mut self) -> Result<SpanOf<P>> {
		let current = self.pendulum.measurement();
		if self.state != State::Running {
			Err(Error::NotRunning)
		} else {
			self.accum += current - self.pivot;
			self.state = State::Stopped;
			Ok(self.accum)
		}
	}

	pub fn lap(&mut self) -> Result<(usize, Elapsed<SpanOf<P>>)> {
		let current = self.pendulum.measurement();

		if self.state != State::Running {
			Err(Error::NotRunning)
		} else {
			let mut split = current - self.pivot;
			split += self.accum;

			let lap = current - self.recent;
			self.recent = current;

			let elapsed = Elapsed::new(split, lap);
			self.memory.push(elapsed);
			Ok((self.memory.len(), elapsed))
		}
	}

	pub fn reset(&mut self) -> SpanOf<P> {
		todo!()
		// let current = self.pendulum.measurement();
		// let ret = self.calc_accum(current);
		//
		// self.accum = SpanOf::<P>::zero();
		// self.state = State::Ready;
		// ret
	}

	pub fn clear(&mut self) -> (M, SpanOf<P>) {
		todo!()
		// let ret = self.reset();
		// let mem = std::mem::take(&mut self.memory);
		// (mem, ret)
	}

	pub fn memory(&self) -> &M {
		&self.memory
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::{Sequence, mock};
	use std::fmt::Debug;

	impl TimeSpan for usize {
		fn zero() -> Self {
			0
		}
	}
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

	type Memory = Vec<Elapsed<usize>>;
	type Fixture = Chronograph<MockDummy, Memory>;

	#[test]
	fn new() {
		let mut mock = MockDummy::new();
		mock.expect_measurement().times(1).returning(|| 42);

		let fixture = Fixture::new(mock, Memory::default());
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
		let mut fixture = Fixture::new(mock, Memory::default());
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
			println!("Payout:{ret}");
			cnt += 1;
			ret
		});

		mock
	}

	#[test]
	fn restart() {
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
		assert_eq!(fixture.restart(), 0);
		assert_eq!(fixture.state, State::Running);
		assert_eq!(fixture.restart(), 1);
		assert_eq!(fixture.state, State::Running);
	}

	#[test]
	fn start() {
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
		assert!(matches!(fixture.start(), Ok(x) if x == 0));
		assert_eq!(fixture.state, State::Running);
		assert!(matches!(fixture.start(), Err(Error::AlreadyRunning)));
		assert_eq!(fixture.state, State::Running);

		_ = fixture.stop().unwrap();
		assert!(matches!(fixture.start(), Ok(x) if x == 2));
	}

	#[test]
	fn stop() {
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
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
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
		assert!(fixture.lap().is_err());
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
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
		_ = fixture.start().unwrap();
		assert_eq!(fixture.reset(), 1);
		assert_eq!(fixture.reset(), 0);
	}

	#[test]
	fn clear() {
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
		_ = fixture.start().unwrap();
		dbg!("splitter");

		for _ in 0..10 {
			_ = fixture.lap().unwrap();
		}
		dbg!("splitter");

		let (mem, ttl) = fixture.clear();
		assert_eq!(mem.len(), 10);
		assert_eq!(ttl, 11);

		for (idx, ela) in mem.iter().enumerate() {
			assert_elapsed(ela, idx + 1, 1);
		}

		assert_eq!(fixture.memory.len(), 0);
	}

	#[test]
	fn memory() {
		let mut fixture = Fixture::new(gen_mock(), Memory::default());
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
			.once()
			.in_sequence(&mut seq)
			.return_const(0usize);

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(1usize);

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(5usize);

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(42usize);

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(45usize);

		let mut fixture = Fixture::new(mock, Memory::default());
		_ = fixture.start().unwrap();
		assert_eq!(fixture.stop().unwrap(), 4);
		assert_eq!(fixture.start().unwrap(), 4);
		assert_eq!(fixture.stop().unwrap(), 7);
	}

	#[test]
	fn lap_complex() {
		fn f(i: usize) -> usize {
			println!("called:{}", i);
			i
		}

		let mut mock = MockDummy::new();
		let mut seq = Sequence::new();

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(0));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(5));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(42));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(48));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(65));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(87));

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.returning(|| f(90));

		let mut fixture = Fixture::new(mock, Memory::default());
		_ = fixture.start().unwrap();
		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 1);
		assert_elapsed(&e, 37, 37);

		let c = fixture.restart();
		assert_eq!(c, 43);

		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 2);
		assert_elapsed(&e, 22, 22);

		let (c, e) = fixture.lap().unwrap();
		assert_eq!(c, 3);
		assert_elapsed(&e, 25, 3);
	}

	#[test]
	fn stop_after_lap_counts_whole_running_segment() {
		let mut mock = MockDummy::new();
		let mut seq = Sequence::new();

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(0usize); // new
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(0usize); // start
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(10usize); // lap
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(15usize); // stop

		let mut fixture = Fixture::new(mock, Memory::default());
		fixture.start().unwrap();
		fixture.lap().unwrap();

		assert_eq!(fixture.stop().unwrap(), 15);
	}

	#[test]
	fn lap_after_resume_includes_accumulated_elapsed() {
		let mut mock = MockDummy::new();
		let mut seq = Sequence::new();

		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(0usize); // new
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(0usize); // start
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(10usize); // stop
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(20usize); // start/resume
		mock.expect_measurement()
			.once()
			.in_sequence(&mut seq)
			.return_const(25usize); // lap

		let mut fixture = Fixture::new(mock, Memory::default());
		fixture.start().unwrap();
		fixture.stop().unwrap();
		fixture.start().unwrap();

		let (_, elapsed) = fixture.lap().unwrap();

		assert_elapsed(&elapsed, 15, 5);
	}
}
