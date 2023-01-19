use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConsumeError {
    UNKNOWN,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ConsumerType {
    String,
}

pub type ConsumerId = uuid::Uuid;
pub type ConsumerRef = (ConsumerType, ConsumerId);

pub struct ConsumerEntry {}

pub type ConsumeResult = Result<bool, Box<dyn std::error::Error>>;

// TODO: Need better name
pub trait Consume<I> {
    fn consume(&mut self, intermediate: I) -> ConsumeResult;
}

pub trait SerializeConsume<I>: Consume<I> + Serialize + Send + 'static {}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::File, io::Write};

    use super::{Consume, ConsumeResult};

    impl<'a> Consume<String> for Box<dyn Write> {
        fn consume(&mut self, intermediate: String) -> ConsumeResult {
            self.write(intermediate.as_bytes()).unwrap();
            Ok(true)
        }
    }

    #[test]
    fn str_consumer() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("string_ouput.txt");
        let file = File::create(file_path.clone()).unwrap();
        let mut job: Box<dyn Write> = Box::new(file);
        let to_write = "print this to a file";
        let result = job.consume(to_write.to_string());
        let file_contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(to_write.to_string(), file_contents);
        assert!(result.unwrap());
    }
}
