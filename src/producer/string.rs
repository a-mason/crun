use super::{Output, Produce};

pub struct StringProducer<'a> {
    pub message: &'a str,
}

impl<'a> Produce<'a, String> for StringProducer<'a> {
    fn run(&self) -> Output<String> {
        Ok(self.message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cron::Schedule;

    use crate::producer::{job::ProducerJob, string::StringProducer};

    #[test]
    fn str_producer() {
        let string_producer = StringProducer {
            message: "cron job ran",
        };
        let s: Schedule = Schedule::from_str("0 15 * * Mar,Jun Mon,Wed,Fri 2017").unwrap();

        let mut job = ProducerJob::with_last_check(s, &string_producer, 1, 0);
        let count = job.check_and_run();
        assert_eq!(1, count.len());
        let count = job.check_and_run();
        assert_eq!(0, count.len());
    }
}
