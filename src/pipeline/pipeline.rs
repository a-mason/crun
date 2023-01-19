use std::marker::PhantomData;

use super::{
    consumer::{Consume, ConsumerRef},
    producer::{Produce, ProducerRef},
};

pub struct Pipeline<P, O, C, I>
where
    P: Produce<O>,
    C: Consume<I>,
{
    producer_ref: ProducerRef,
    producer: P,
    consumer_ref: ConsumerRef,
    consumer: C,
    _output: PhantomData<O>,
    _input: PhantomData<I>,
}

impl<P, O, C, I> Pipeline<P, O, C, I>
where
    P: Produce<O>,
    C: Consume<I>,
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
            _input: PhantomData,
            _output: PhantomData,
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
