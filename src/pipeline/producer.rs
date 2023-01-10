use super::Intermediate;

pub type ProducerId = (ProducerType, uuid::Uuid);

#[derive(PartialEq, Debug)]
pub enum ProducerType {
    String,
}

pub trait Produce<I: Intermediate> {
    fn produce(&self) -> I;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cron::Schedule;

    use crate::pipeline::{producer::ProducerType, Intermediate};

    use super::Produce;

    struct StringProducer<'a> {
        message: &'a str,
    }
    impl<'a> StringProducer<'a> {
        pub fn new(message: &'a str) -> Self {
            StringProducer { message: message }
        }
    }

    impl Intermediate for String {}

    impl<'a> Produce<String> for StringProducer<'a> {
        fn produce(&self) -> String {
            self.message.to_string()
        }
    }

    #[test]
    fn str_producer() {
        let str_producer = StringProducer::new("this is a string");

        assert_eq!("this is a string", str_producer.produce());
    }
}
