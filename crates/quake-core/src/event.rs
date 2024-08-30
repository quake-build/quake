use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

use async_trait::async_trait;
use chrono::{DateTime, Local};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::engine::Engine;
use crate::task::TaskStatus;
use crate::Result;

#[derive(Debug)]
pub struct Event<E: Engine> {
    pub kind: EventKind<E>,
    pub task: Option<()>,
}

impl<E: Engine> Event<E> {
    #[inline(always)]
    pub const fn new(kind: EventKind<E>, task: Option<()>) -> Self {
        Self { kind, task }
    }

    #[inline(always)]
    pub const fn with_task(kind: EventKind<E>, task: ()) -> Self {
        Self {
            kind,
            task: Some(task),
        }
    }
}

impl<E: Engine> Deref for Event<E> {
    type Target = EventKind<E>;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

#[derive(Debug)]
pub enum EventKind<E: Engine> {
    Log(String),
    Error(E::Error),
    Other(E::Event),
}
