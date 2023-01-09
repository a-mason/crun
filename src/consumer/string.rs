use std::{io::Write, sync::Mutex};

use super::{CResult, Consume};

pub struct StringConsumer<'a> {
    pub output_location: Mutex<&'a mut dyn Write>,
}

impl<'a> Consume<'a, String> for StringConsumer<'a> {
    fn run(&self, str: String) -> CResult<bool> {
        self.output_location
            .lock()
            .unwrap()
            .write(str.as_bytes())
            .unwrap();
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::File, sync::Mutex};

    use crate::consumer::{string::StringConsumer, ConsumerJob};

    #[test]
    fn str_consumer() {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("string_ouput.txt");
        let mut file = File::create(file_path.clone()).unwrap();
        let print_job = StringConsumer {
            output_location: Mutex::new(&mut file),
        };
        let job = ConsumerJob::new(&print_job);
        let to_write = "print this to a file";
        let result = job.consume(to_write.to_string());
        let file_contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(to_write.to_string(), file_contents);
        assert!(result.unwrap());
    }
}
