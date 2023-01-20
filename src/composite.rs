use super::{
    consumer::{Consume, ConsumerRef},
    producer::{Produce, ProducerRef},
};

pub struct Composite<P, C>
where
    P: Produce,
    C: Consume,
{
    producer_ref: ProducerRef,
    producer: P,
    consumer_ref: ConsumerRef,
    consumer: C,
}

impl<P, C> Composite<P, C>
where
    P: Produce,
    C: Consume,
{
    pub fn new(
        producer_ref: ProducerRef,
        producer: P,
        consumer_ref: ConsumerRef,
        consumer: C,
    ) -> Self {
        Self {
            producer_ref,
            producer,
            consumer_ref,
            consumer,
        }
    }

    pub fn producer_ref(&self) -> &ProducerRef {
        &self.producer_ref
    }

    pub fn consumer_ref(&self) -> &ConsumerRef {
        &self.consumer_ref
    }

    pub fn producer(&self) -> &P {
        &self.producer
    }

    pub fn consumer(&self) -> &C {
        &self.consumer
    }
}
