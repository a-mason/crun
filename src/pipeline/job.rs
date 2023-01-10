use std::{cmp::Ordering, fmt};

use chrono::{TimeZone, Utc};
use cron::Schedule;
use uuid::Uuid;

use super::{consumer::ConsumerRef, producer::ProducerRef};

pub type JobId = uuid::Uuid;

pub struct Job {
    id: JobId,
    schedule: Schedule,
    last_check: Option<i64>,
    next_run: Option<i64>,
    limit_runs: usize,
    producer: ProducerRef,
    consumer: ConsumerRef,
}

impl Eq for Job {}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.schedule == other.schedule
            && self.last_check == other.last_check
            && self.next_run == other.next_run
            && self.limit_runs == other.limit_runs
            && self.producer == other.producer
            && self.consumer == other.consumer
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want to always get the Producer that needs to be run soonest
        // And break the tie with the Producer that was least recently run
        other
            .next_run
            .cmp(&self.next_run)
            .then_with(|| other.last_check.cmp(&self.last_check))
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Job")
            .field("producer", &self.producer)
            .field("consumer", &self.consumer)
            .field("schedule", &self.schedule)
            .field("last_check", &self.last_check)
            .field("next_run", &self.next_run)
            .field("limit_runs", &self.limit_runs)
            .finish()
    }
}

impl Job {
    pub fn new(
        schedule: Schedule,
        producer: ProducerRef,
        consumer: ConsumerRef,
        limit_runs: usize,
    ) -> Self {
        // TODO: validate limit_runs is greater than 0
        Job {
            id: uuid::Uuid::new_v4(),
            producer,
            consumer,
            last_check: None,
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            schedule,
        }
    }

    pub fn with_last_check(
        schedule: Schedule,
        producer: ProducerRef,
        consumer: ConsumerRef,
        limit_runs: usize,
        last_check: i64,
    ) -> Self {
        // TODO: validate limit_runs is greater than 0
        Job {
            id: uuid::Uuid::new_v4(),
            producer,
            consumer,
            last_check: Some(last_check),
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            schedule,
        }
    }

    pub fn check(&mut self) -> usize {
        let now = chrono::Utc::now().timestamp();
        let last_check = self.last_check.unwrap_or(now);
        let mut runs = 0;
        for event in self
            .schedule
            .after(&chrono::Utc.timestamp(last_check, 0))
            .take(self.limit_runs)
        {
            if event.timestamp() > now {
                break;
            }
            runs += 1;
        }
        self.last_check = Some(now);
        runs
    }

    pub fn next_run(&self) -> Option<i64> {
        self.schedule
            .upcoming(chrono::Utc)
            .next()
            .map(|t| t.timestamp())
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cron::Schedule;

    use crate::pipeline::{consumer::ConsumerType, job::Job, producer::ProducerType};

    #[test]
    fn schedule() {
        let s: Schedule = Schedule::from_str("0 15 * * Mar,Jun Mon,Wed,Fri 2017").unwrap();
        let p = (ProducerType::String, uuid::Uuid::new_v4());
        let c = (ConsumerType::String, uuid::Uuid::new_v4());
        let mut job = Job::with_last_check(s, p, c, 1, 0);
        let count = job.check();
        assert_eq!(1, count);
        let count = job.check();
        assert_eq!(0, count);
    }
}
