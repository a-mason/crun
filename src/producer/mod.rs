pub mod job;
pub mod string;

pub enum ProduceError {
    UNKNOWN,
}

pub type Output<T> = std::result::Result<T, ProduceError>;

pub trait Produce<'a, T> {
    fn run(&self) -> Output<T>;
}
