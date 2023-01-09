pub mod string;

#[derive(Debug)]
pub enum ConsumeError {
    UNKNOWN,
}

// TODO: Need better name
pub type CResult<T> = std::result::Result<T, ConsumeError>;

pub trait Consume<'a, T> {
    fn run(&self, t: T) -> CResult<bool>;
}
pub struct ConsumerJob<'a, T> {
    consumer: &'a dyn Consume<'a, T>,
}

impl<'a, T> ConsumerJob<'a, T> {
    pub fn new(consumer: &'a dyn Consume<'a, T>) -> Self {
        ConsumerJob { consumer }
    }

    pub fn consume(&self, t: T) -> CResult<bool> {
        self.consumer.run(t)
    }
}
