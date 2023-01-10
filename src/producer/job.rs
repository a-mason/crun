use std::{cmp::Ordering, fmt};

use chrono::{TimeZone, Utc};
use cron::Schedule;
use uuid::Uuid;

use super::{Output, Produce, ProducerId, Intermediate};

pub struct ProducerJob<I: Intermediate> {
    schedule: Schedule,
    producer: Box<dyn Produce<I>>,
    last_check: Option<i64>,
    next_run: Option<i64>,
    limit_runs: usize,
    id: ProducerId,
}

impl<I: Intermediate> Eq for ProducerJob<I> {}

impl<I: Intermediate> PartialEq for ProducerJob<I> {
    fn eq(&self, other: &Self) -> bool {
        self.schedule == other.schedule
            && self.last_check == other.last_check
            && self.next_run == other.next_run
            && self.limit_runs == other.limit_runs
            && self.id == other.id
    }
}

impl<I: Intermediate> Ord for ProducerJob<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want to always get the Producer that needs to be run soonest
        // And break the tie with the Producer that was least recently run
        other
            .next_run
            .cmp(&self.next_run)
            .then_with(|| other.last_check.cmp(&self.last_check))
    }
}

impl< I: Intermediate> PartialOrd for ProducerJob<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: Intermediate> fmt::Debug for ProducerJob<I> {
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

impl<I: Intermediate> ProducerJob<I> {
    pub fn new(schedule: Schedule, producer: Box<dyn Produce<I>>, limit_runs: usize) -> Self {
        // TODO: validate limit_runs is greater than 0
        ProducerJob {
            id: Uuid::new_v4(),
            last_check: None,
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            producer: producer,
            schedule,
        }
    }

    pub fn with_last_check(
        schedule: Schedule,
        producer: Box<dyn Produce<I>>,
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

    pub fn check_and_run(&mut self) -> Vec<Output<I>> {
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

    pub fn next_run(&self) -> Option<i64> {
        self.schedule.upcoming(chrono::Utc).next().map(|t| { t.timestamp() })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
