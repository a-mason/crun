use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};

use crate::pipeline::{
    composite::Composite,
    consumer::{Consume, ConsumerRef},
    job::{Job, JobId},
    producer::{Produce, ProducerRef},
};

pub type JobHeap = BinaryHeap<(i64, JobId)>;

pub type JobMap = HashMap<JobId, Job>;

pub type ProducerMap = HashMap<ProducerRef, Vec<u8>>;

pub type ConsumerMap = HashMap<ConsumerRef, Vec<u8>>;

pub struct MemoryJobStore {
    next_job: JobHeap,
    jobs: JobMap,
    producers: ProducerMap,
    consumers: ConsumerMap,
}

impl MemoryJobStore {
    pub fn new() -> Self {
        MemoryJobStore {
            next_job: BinaryHeap::new(),
            jobs: HashMap::new(),
            producers: HashMap::new(),
            consumers: HashMap::new(),
        }
    }

    fn peek(&self) -> Option<JobId> {
        self.next_job.peek().map(|i| i.1)
    }

    fn pop(&mut self) -> Option<JobId> {
        self.next_job.pop().map(|i| i.1)
    }

    pub fn insert<P, C>(&mut self, job: Job, composite: Composite<P, C>)
    where
        P: Produce + Serialize,
        C: Consume + Serialize,
    {
        match job.next_run() {
            Some(timestamp) => {
                self.next_job.push((timestamp, job.id()));
                Some(timestamp)
            }
            None => None,
        };
        self.jobs.insert(job.id(), job);
        self.producers.insert(
            composite.producer_ref().clone(),
            serde_json::to_vec(composite.producer()).unwrap(),
        );
        self.consumers.insert(
            composite.consumer_ref().clone(),
            serde_json::to_vec(composite.consumer()).unwrap(),
        );
    }

    pub fn get_producer<T: Produce + for<'de> Deserialize<'de>>(
        &self,
        id: &ProducerRef,
    ) -> Option<T> {
        self.producers
            .get(id)
            .map(|p| serde_json::from_slice(p).unwrap())
    }

    pub fn get_consumer<T: Consume + for<'de> Deserialize<'de>>(
        &self,
        id: &ConsumerRef,
    ) -> Option<T> {
        self.consumers
            .get(id)
            .map(|c| serde_json::from_slice(c).unwrap())
    }
}
