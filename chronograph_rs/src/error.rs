use thiserror::Error;

pub type Result<T> = std::result::Result<T,Error>;

#[derive(Error,Debug)]
pub enum Error{
	#[error("Chronograph is already running")]
	AlreadyRunning,
	#[error("Chronograph is not running")]
	NotRunning,
}