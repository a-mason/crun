use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::Produce;

    struct StringProducer<'a> {
        message: &'a str,
    }
    impl<'a> StringProducer<'a> {
        pub fn new(message: &'a str) -> Self {
            StringProducer { message }
        }
    }

    impl<'a> Produce for StringProducer<'a> {
        type Output = String;
        fn produce(&self) -> Self::Output {
            self.message.to_string()
        }
    }

    #[test]
    fn str_producer() {
        let str_producer = StringProducer::new("this is a string");

        assert_eq!("this is a string", str_producer.produce());
    }
}
