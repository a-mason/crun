use std::collections::{BinaryHeap, HashMap};

use crate::{
    consumer::{job::Consumer, ConsumerId},
    producer::{job::ProducerJob, NextRun, ProducerId}, Intermediate,
};

pub type ProducerHeap = BinaryHeap<NextRun>;

pub type ProducerMap<T> = HashMap<ProducerId, ProducerJob<T>>;

pub type ProducerConsumerMap = HashMap<ProducerId, ConsumerId>;

pub type ConsumerMap<T> = HashMap<ConsumerId, Consumer<T>>;

pub struct MemoryJobStore<T: Intermediate> {
    next_producer: ProducerHeap,
    producers: ProducerMap<T>,
    consumers: ConsumerMap<T>,
    prod_cons: ProducerConsumerMap,
}

impl<T: Intermediate> MemoryJobStore<T> {
    pub fn new() -> Self {
        MemoryJobStore {
            next_producer: BinaryHeap::new(),
            producers: HashMap::new(),
            consumers: HashMap::new(),
            prod_cons: HashMap::new(),
        }
    }

    fn peek(&self) -> Option<ProducerId> {
        self.next_producer.peek().map(|i| { i.1 })
    }

    fn pop(&mut self) -> Option<ProducerId> {
        self.next_producer.pop().map(|i| { i.1 })
    }

    pub fn insert(&mut self, producer_job: ProducerJob<T>, consumer_job: Consumer<T>) {
        match producer_job.next_run() {
            Some(timestamp) => {
                self.next_producer.push((timestamp, producer_job.id()));
                Some(timestamp)
            },
            None => None
        };
        self.prod_cons.insert(producer_job.id(), consumer_job.id());
        self.producers.insert(producer_job.id(), producer_job);
        self.consumers.insert(consumer_job.id(), consumer_job);
    }

    pub fn next(&mut self) -> Option<&ProducerJob<T>> {
        self.peek().and_then(|producer_id| {
            self.producers.get(&producer_id)
        })
    }

    pub fn get_producer(&self, id: &ProducerId) -> Option<&ProducerJob<T>> {
        self.producers.get(id)
    }

    pub fn get_consumer(&self, id: &ConsumerId) -> Option<&Consumer<T>> {
        self.consumers.get(id)
    }
}
