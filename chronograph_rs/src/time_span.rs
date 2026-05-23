use std::time::Duration;

pub trait TimeSpan: Default + Sized + Clone + Copy {}

impl TimeSpan for Duration {}
