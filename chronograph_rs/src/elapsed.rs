use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Elapsed {
    recorded_at: Instant,
    split: Duration,
    lap: Duration,
}

impl Elapsed {
    pub fn new(recorded_at: Instant, recent: Instant, pivot: Instant) -> Self {
        debug_assert!(
            recorded_at >= recent,
            "recorded_at must be after or equal to recent"
        );
        debug_assert!(recent >= pivot, "recent must be after or equal to pivot");
        Self {
            recorded_at,
            split: recorded_at - pivot,
            lap: recorded_at - recent,
        }
    }

    pub fn recorded_at(&self) -> Instant {
        self.recorded_at
    }

    pub fn lap(&self) -> Duration {
        self.lap
    }

    pub fn split(&self) -> Duration {
        self.split
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    #[test]
    fn new() {
        let recorded_at = Instant::now();
        let lap = recorded_at.sub(Duration::from_secs(1));
        let split = recorded_at.sub(Duration::from_secs(2));

        let fixture = Elapsed::new(recorded_at, lap, split);

        assert_eq!(fixture.recorded_at, recorded_at);
        assert_eq!(fixture.lap, Duration::from_secs(1));
        assert_eq!(fixture.split, Duration::from_secs(2));

        let zero = Elapsed::new(recorded_at, recorded_at, recorded_at);
        assert_eq!(zero.recorded_at, recorded_at);
        assert_eq!(zero.lap, Duration::default());
        assert_eq!(zero.split, Duration::default());
    }

    #[test]
    #[should_panic]
    fn invalid_recent_new() {
        let recorded_at = Instant::now();
        let recent = recorded_at + Duration::from_secs(1);
        let pivot = recorded_at - Duration::from_secs(10);

        _ = Elapsed::new(recorded_at, recent, pivot);
    }

    #[test]
    #[should_panic]
    fn invalid_pivot_new() {
        let recorded_at = Instant::now();
        let recent = recorded_at - Duration::from_secs(1);
        let pivot = recorded_at + Duration::from_secs(10);

        _ = Elapsed::new(recorded_at, recent, pivot);
    }

    #[test]
    #[should_panic]
    fn invalid_discrepancy_new() {
        let recorded_at = Instant::now();
        let recent = recorded_at - Duration::from_secs(5);
        let pivot = recorded_at - Duration::from_secs(3);

        _ = Elapsed::new(recorded_at, recent, pivot);
    }

    #[test]
    fn recorded_at() {
        let recorded_at = Instant::now();
        let recent = recorded_at - Duration::from_secs(1);
        let pivot = recorded_at - Duration::from_secs(10);

        let fixture = Elapsed::new(recorded_at, recent, pivot);

        assert_eq!(fixture.recorded_at(), recorded_at);
    }

    #[test]
    fn lap() {
        let recorded_at = Instant::now();
        let recent = recorded_at - Duration::from_secs(1);
        let pivot = recorded_at - Duration::from_secs(10);

        let fixture = Elapsed::new(recorded_at, recent, pivot);

        assert_eq!(fixture.lap(), Duration::from_secs(1));
    }

    #[test]
    fn split() {
        let recorded_at = Instant::now();
        let recent = recorded_at - Duration::from_secs(1);
        let pivot = recorded_at - Duration::from_secs(10);

        let fixture = Elapsed::new(recorded_at, recent, pivot);

        assert_eq!(fixture.split(), Duration::from_secs(10));
    }
}
