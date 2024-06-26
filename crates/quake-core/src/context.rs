#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Result;

// pub struct CtxHandle {}

pub struct Context {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            subscribers: vec![],
        }
    }

    pub fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber)
    }

    fn send_event(&self, event: &Event) -> Result<()> {
        for sub in &self.subscribers {
            sub.recv_event(&self, event)?;
        }

        Ok(())
    }
}

pub struct ContextBuilder {
    inner: Box<Context>, // TODO Box, Arc, ? need to figure out design
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            inner: Box::new(Context::new()),
        }
    }

    pub fn with_subscriber(mut self, subscriber: Box<dyn Subscriber>) -> Self {
        self.inner.add_subscriber(subscriber);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Event {}

pub trait Subscriber {
    fn recv_event(&self, ctx: &Context, event: &Event) -> Result<()>;
}
