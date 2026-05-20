use std::time::{Instant,Duration};

#[derive(Debug,Clone)]
pub struct Elapsed {
	pivot:Instant,
	split:Duration,
	lap:Duration,
}

impl Elapsed {
	pub fn new(current:Instant,recent:Instant,pivot:Instant) -> Self {
		todo!()
	}
	
	pub fn pivot(&self)->Instant {
		todo!()
	}
	
	pub fn lap(&self)->Duration {
		todo!()
	}
	
	pub fn split(&self)->Duration {
		todo!()
	}
}


#[cfg(test)]
mod tests {
	use std::ops::Add;
	use super::*;
	use std::panic::catch_unwind;
	
	#[test]
	fn new() {
		let piv=Instant::now();
		let lap=piv.add(Duration::from_secs(1));
		let split=piv.add(Duration::from_secs(2));
		
		let fixture=Elapsed::new(piv,lap,split);
		
		assert_eq!(fixture.pivot(),piv);
		assert_eq!(fixture.lap(),lap-piv);
		assert_eq!(fixture.split(),split-piv);
		
		let zero=Elapsed::new(piv,piv,piv);
		assert_eq!(zero.pivot(),piv);
		assert_eq!(zero.lap(),Duration::default());
		assert_eq!(zero.split(),Duration::default());
	}
	
	#[test]
	fn invalid_new() {
		
	}
	
}