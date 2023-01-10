use crate::Intermediate;

pub mod job;
pub mod string;

pub enum ProductionError {
    UNKNOWN,
}

pub type ProducerId = uuid::Uuid;


pub type Output<P> = std::result::Result<P, ProductionError>;

pub trait Produce<P: Intermediate> {
    fn run(&self) -> Output<P>;
}

pub type NextRun = (i64, ProducerId);


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

        let mut job = ProducerJob::with_last_check(s, Box::new(string_producer), 1, 0);
        let count = job.check_and_run();
        assert_eq!(1, count.len());
        let count = job.check_and_run();
        assert_eq!(0, count.len());
    }
}
