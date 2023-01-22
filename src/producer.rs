use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub enum OutputError {
    UNKNOWN,
}

pub type Output<I> = std::result::Result<I, OutputError>;

pub type ProducerId = uuid::Uuid;

pub type ProducerRef = (ProducerType, ProducerId);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ProducerType {
    String,
}

pub trait Produce {
    type Output;
    fn produce(&self) -> Self::Output;
}

#[async_trait]
pub trait AsyncProduce {
    type Output;
    async fn produce(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::Produce;

    impl<'a> Produce for &'a str {
        type Output = String;
        fn produce(&self) -> Self::Output {
            self.to_string()
        }
    }

    #[test]
    fn str_producer() {
        assert_eq!("this is a string", "this is a string".produce());
    }
}
