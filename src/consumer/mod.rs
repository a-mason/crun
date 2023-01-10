use uuid::Uuid;

use crate::{Intermediate};

type ConsumerFn<I> = dyn (FnMut(I)) + Send + Sync;

#[derive(Debug)]
pub enum ConsumeError {
    UNKNOWN,
}

pub type ConsumerId = uuid::Uuid;

// TODO: Need better name
pub type CResult = std::result::Result<bool, ConsumeError>;

pub struct Consumer<I: Intermediate> {
    runner: Box<ConsumerFn<I>>,
    id: ConsumerId,
}

impl<I: Intermediate> Consumer<I> {
    pub fn new(consume: Box<ConsumerFn<I>>) -> Self {
        Consumer {
            id: uuid::Uuid::new_v4(),
            runner: consume,
        }
    }

    pub fn consume(&mut self, intermediate: I) -> CResult {
        (self.runner)(intermediate);
        Ok(true)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}


#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::File, io::Write};

    use crate::consumer::Consumer;

    #[test]
    fn str_consumer() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("string_ouput.txt");
        let mut file = File::create(file_path.clone()).unwrap();
        let mut job = Consumer::new(Box::new(move |i: String| {
            file.write(i.as_bytes()).unwrap();
        }));
        let to_write = "print this to a file";
        let result = job.consume(to_write.to_string());
        let file_contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(to_write.to_string(), file_contents);
        assert!(result.unwrap());
    }
}
