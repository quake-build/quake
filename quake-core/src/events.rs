use std::fmt::Debug;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::engine::{Engine, EngineStatus};
use crate::tasks::TaskStatus;
use crate::Result;

#[async_trait]
pub trait Subscriber<'ctx, E: Engine<'ctx>> {
    async fn recv_event(&self, ctx: &'ctx Context<'ctx, E>, event: &Event<'ctx, E>) -> Result<()>;
}

pub enum Event<'ctx, E: Engine<'ctx>> {
    Engine(EngineEvent<'ctx, E>),
    Task(TaskEvent<'ctx, E>),
    Log(LogEvent<'ctx, E>),
    Fatal(Error),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum EngineEvent<'ctx, E: Engine<'ctx>> {
    StatusUpdate(Update<EngineStatus>),
    Error(E::Error),
    Custom(E::Event),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TaskEvent<'ctx, E: Engine<'ctx>> {
    StatusUpdate(Update<TaskStatus>),
    Error(E::TaskError),
    Custom(E::TaskEvent),
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogEvent<'ctx, E: Engine<'ctx>> {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub task: Option<E::TaskId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Update<T> {
    pub old: T,
    pub new: T,
}
