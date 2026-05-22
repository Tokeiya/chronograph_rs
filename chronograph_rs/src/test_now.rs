#![allow(clippy::disallowed_methods)]
#![cfg(test)]

use std::cell::Cell;
use std::ops::Add;
use std::time::{Duration, Instant};

thread_local! {
	static PIVOT:Cell<Instant> = Cell::new(Instant::now());
	static COUNT:Cell<u64> = const { Cell::new(0) };
}
pub(crate) fn now() -> Instant {
	let pivot = PIVOT.get();
	let tmp = COUNT.get();
	COUNT.set(tmp + 1);

	pivot.add(Duration::from_secs(tmp))
}

pub(crate) fn pivot() -> Instant {
	PIVOT.get()
}

pub(crate) fn count() -> u64 {
	COUNT.get()
}
pub(crate) fn reset() {
	PIVOT.with(|now| {
		now.set(Instant::now());
	});

	COUNT.with(|count| {
		count.set(0);
	});
}
