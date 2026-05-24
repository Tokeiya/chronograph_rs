use super::lap_memory::Memory;
use crate::measurement::elapsed::Elapsed;
use crate::measurement::time_span::TimeSpan;

impl<T: TimeSpan> Memory<T> for Vec<Elapsed<T>> {
	type Iter<'a>
		= std::slice::Iter<'a, Elapsed<T>>
	where
		Self: 'a,
		T: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}

	fn push(&mut self, value: Elapsed<T>) {
		self.push(value);
	}

	fn len(&self) -> usize {
		self.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::measurement::time_span::TimeSpan;
	type Fixture = Vec<Elapsed<usize>>;

	#[test]
	fn iter() {
		let mut fixture = Fixture::default();
		for i in 0..10 {
			fixture.push(Elapsed::new(i, i));
		}

		let iter = <Fixture as Memory<usize>>::iter(&fixture);

		for (idx, ela) in iter.as_slice().iter().enumerate() {
			assert_eq!(idx, ela.lap());
			assert_eq!(idx, ela.split());
		}
	}

	#[test]
	fn push() {
		let mut fixture = Fixture::default();
		for i in 0..10 {
			<Fixture as Memory<usize>>::push(&mut fixture, Elapsed::new(i, i));
		}

		for (i, e) in fixture.iter().enumerate() {
			assert_eq!(i, e.lap());
			assert_eq!(i, e.split());
		}
	}

	#[test]
	fn len() {
		let mut fixture = Fixture::default();
		assert_eq!(0, fixture.len());

		for i in 0..10 {
			assert_eq!(<Fixture as Memory<usize>>::len(&fixture), i);
			fixture.push(Elapsed::new(i, i));
		}

		assert_eq!(<Fixture as Memory<usize>>::len(&fixture), 10);
	}
}
