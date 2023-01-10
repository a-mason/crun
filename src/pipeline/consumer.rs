use super::Intermediate;

type ConsumerFn<I> = dyn (FnMut(I)) + Send + Sync;

#[derive(Debug)]
pub enum ConsumeError {
    UNKNOWN,
}

#[derive(PartialEq, Debug)]
pub enum ConsumerType {
    String,
}

pub type ConsumerId = (ConsumerType, uuid::Uuid);

pub struct ConsumerEntry {}

pub type ConsumeResult = Result<bool, Box<dyn std::error::Error>>;

// TODO: Need better name
pub trait Consume<I: Intermediate> {
    fn consume(&mut self, intermediate: I) -> ConsumeResult;
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::File, io::Write};

    use super::{Consume, ConsumeResult};

    struct StringConsumer {
        output_location: Box<dyn Write>,
    }
    impl StringConsumer {
        pub fn new(output_location: Box<dyn Write>) -> Self {
            StringConsumer { output_location }
        }
    }

    impl<'a> Consume<String> for StringConsumer {
        fn consume(&mut self, intermediate: String) -> ConsumeResult {
            self.output_location.write(intermediate.as_bytes());
            Ok(true)
        }
    }

    #[test]
    fn str_consumer() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("string_ouput.txt");
        let mut file = File::create(file_path.clone()).unwrap();
        let mut job = StringConsumer::new(Box::new(file));
        let to_write = "print this to a file";
        let result = job.consume(to_write.to_string());
        let file_contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(to_write.to_string(), file_contents);
        assert!(result.unwrap());
    }
}
