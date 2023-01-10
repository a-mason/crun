// use std::collections::{BinaryHeap, HashMap};

// use crate::{
//     pipeline::{
//         consumer::{ConsumerId},
//         producer::{ProducerId},
//     },
// };

// pub type ProducerHeap = BinaryHeap<(i64, ProducerId)>;

// pub type ProducerMap = HashMap<ProducerId, Producer>;

// pub type ProducerConsumerMap = HashMap<ProducerId, ConsumerId>;

// pub type ConsumerMap = HashMap<ConsumerId, Consumer<Box<dyn Intermediate>>>;

// pub struct MemoryJobStore {
//     next_producer: ProducerHeap,
//     producers: ProducerMap,
//     consumers: ConsumerMap,
//     prod_cons: ProducerConsumerMap,
// }

// impl MemoryJobStore {
//     pub fn new() -> Self {
//         MemoryJobStore {
//             next_producer: BinaryHeap::new(),
//             producers: HashMap::new(),
//             consumers: HashMap::new(),
//             prod_cons: HashMap::new(),
//         }
//     }

//     fn peek(&self) -> Option<ProducerId> {
//         self.next_producer.peek().map(|i| i.1)
//     }

//     fn pop(&mut self) -> Option<ProducerId> {
//         self.next_producer.pop().map(|i| i.1)
//     }

//     pub fn insert(&mut self, producer_job: Producer<I>, consumer_job: Consumer<I>) {
//         match producer_job.next_run() {
//             Some(timestamp) => {
//                 self.next_producer.push((timestamp, producer_job.id()));
//                 Some(timestamp)
//             }
//             None => None,
//         };
//         self.prod_cons.insert(producer_job.id(), consumer_job.id());
//         self.producers.insert(producer_job.id(), producer_job);
//         self.consumers.insert(consumer_job.id(), consumer_job);
//     }

//     pub fn next(&mut self) -> Option<&Producer<I>> {
//         self.peek()
//             .and_then(|producer_id| self.producers.get(&producer_id))
//     }

//     pub fn get_producer(&self, id: &ProducerId) -> Option<&Producer<I>> {
//         self.producers.get(id)
//     }

//     pub fn get_consumer(&self, id: &ConsumerId) -> Option<&Consumer<I>> {
//         self.consumers.get(id)
//     }
// }
