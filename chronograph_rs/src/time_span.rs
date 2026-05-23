use std::ops::{Add, Sub};

pub trait TimeSpan: Sub + Add + Sized {}
