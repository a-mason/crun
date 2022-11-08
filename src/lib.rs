use std::fmt;

use chrono::{Utc, TimeZone};
use cron::Schedule;
use uuid::Uuid;

struct PrintRun<'a> {
    message: &'a str
}

impl <'a> Run for PrintRun<'a> {
    fn run(&self) -> bool {
        println!("{}", self.message);
        true
    }
}

pub trait Run {
    fn run(&self) -> bool;
}

pub struct Job<'a> {
    schedule: Schedule,
    run: Box<dyn Run + 'a>,
    last_check: Option<i64>,
    next_run: Option<i64>,
    limit_runs: usize,
    id: Uuid,
}

impl<'a> fmt::Debug for Job<'a> {
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

impl<'a> Job<'a> {
    pub fn new(schedule: Schedule, run: Box<dyn Run + 'a>, limit_runs: usize) -> Self {
        // TODO: validate limit_runs is greater than 0
        Job {
            id: Uuid::new_v4(),
            last_check: None,
            next_run: schedule.upcoming(Utc).next().map(|f| f.timestamp()),
            limit_runs,
            run,
            schedule,
        }
    }

    pub fn check_and_run(&mut self) -> usize {
        let now = chrono::Utc::now().timestamp();
        if self.last_check.is_none() {
            self.last_check = Some(now);
        }
        let mut run_count = 0;
        for event in self
            .schedule
            .after(&chrono::Utc.timestamp(self.last_check.unwrap(), 0))
            .take(self.limit_runs)
        {
            if event.timestamp() > now {
                break;
            }
            (self.run).run();
            run_count += 1;
        };
        run_count
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        let print_job = PrintRun {
            message: "cron job ran",
        };
        let s: Schedule = Schedule::from_str("0 15 6,8,10 * Mar,Jun Fri 2017").unwrap();
        let mut job = Job::new(s, Box::new(print_job), 1);
        let count = job.check_and_run();
        println!("Job ran {} times", count);
        let count = job.check_and_run();
        println!("Job ran {} times", count);
    }
}
