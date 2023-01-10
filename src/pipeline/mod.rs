pub mod consumer;
pub mod job;
pub mod producer;

pub enum OutputError {
    UNKNOWN,
}

pub type Output<I> = std::result::Result<I, OutputError>;

pub trait Intermediate {}
