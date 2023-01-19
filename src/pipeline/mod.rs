pub mod consumer;
pub mod job;
pub mod pipeline;
pub mod producer;

pub enum OutputError {
    UNKNOWN,
}

pub type Output<I> = std::result::Result<I, OutputError>;
