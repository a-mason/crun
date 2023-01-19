use serde::{Deserialize, Serialize};

pub type ProducerId = uuid::Uuid;

pub type ProducerRef = (ProducerType, ProducerId);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ProducerType {
    String,
}

pub trait Produce<P> {
    fn produce(&self) -> P;
}

#[cfg(test)]
mod tests {
    use super::Produce;

    impl<'a> Produce<String> for &'a str {
        fn produce(&self) -> String {
            self.to_string()
        }
    }

    impl<'a> Produce<usize> for &'a str {
        fn produce(&self) -> usize {
            self.len()
        }
    }

    #[test]
    fn str_producer() {
        let produced: String = "this is a string".produce();
        assert_eq!("this is a string", produced);
    }
}
