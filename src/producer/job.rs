use std::fmt;

use chrono::{TimeZone, Utc};
use cron::Schedule;
use uuid::Uuid;

use super::{Output, Produce};

pub struct ProducerJob<'a, T> {
    schedule: Schedule,
    producer: &'a dyn Produce<'a, T>,
    last_check: Option<i64>,
    next_run: Option<i64>,
    limit_runs: usize,
    id: Uuid,
}

impl<'a, T> fmt::Debug for ProducerJob<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Job")
            .field("id", &self.id)
            .field("schedule", &self.schedule)
            .field("last_check", &self.last_check)
            .field("next_run", &self.next_run)
            .field("limit_runs", &self.limit_runs)
            .finish()
    }
}

impl<'a, T> ProducerJob<'a, T> {
    pub fn new(schedule: Schedule, producer: &'a dyn Produce<'a, T>, limit_runs: usize) -> Self {
        // TODO: validate limit_runs is greater than 0
        ProducerJob {
            id: Uuid::new_v4(),
            last_check: None,
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            producer,
            schedule,
        }
    }

    pub fn with_last_check(
        schedule: Schedule,
        producer: &'a dyn Produce<'a, T>,
        limit_runs: usize,
        last_check: i64,
    ) -> Self {
        // TODO: validate limit_runs is greater than 0
        ProducerJob {
            id: Uuid::new_v4(),
            last_check: Some(last_check),
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            producer,
            schedule,
        }
    }

    pub fn check_and_run(&mut self) -> Vec<Output<T>> {
        let now = chrono::Utc::now().timestamp();
        let last_check = self.last_check.unwrap_or(now);
        let mut run_results = Vec::new();
        for event in self
            .schedule
            .after(&chrono::Utc.timestamp(last_check, 0))
            .take(self.limit_runs)
        {
            if event.timestamp() > now {
                break;
            }
            run_results.push((self.producer).run());
        }
        self.last_check = Some(now);
        run_results
    }
}
