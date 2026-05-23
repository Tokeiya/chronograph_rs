use super::moment::Moment;

pub trait Pendulum<T, U> {
    fn measurement(&self);
}
