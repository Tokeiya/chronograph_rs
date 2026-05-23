use std::ops::Add;
use std::time::Duration;

pub trait TimeSpan: Add + Default + Sized + Clone + Copy {}

impl TimeSpan for Duration {}
