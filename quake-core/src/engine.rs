use std::fmt::Debug;
use std::hash::Hash;

use async_trait::async_trait;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::context::{Context, ContextBuilder, TaskContext};
use crate::tasks::{TaskGraph, TaskStatus};

#[async_trait]
pub trait Engine<'ctx>: 'ctx + Sized {
    #[cfg(not(feature = "serde"))]
    type TaskId: Eq + Hash;
    #[cfg(feature = "serde")]
    type TaskId: Eq + Hash + Serialize + for<'de> Deserialize<'de>;
   
    type Task: Debug + Clone;

    // TODO clean up when a better alternative is sufficiently stable
    // (trait alias, type impl alias, etc.)

    #[cfg(not(feature = "serde"))]
    type Event: Debug + Clone;
    #[cfg(feature = "serde")]
    type Event: Debug + Clone + Serialize + for<'de> Deserialize<'de>;

    #[cfg(not(feature = "serde"))]
    type TaskEvent;
    #[cfg(feature = "serde")]
    type TaskEvent: Debug + Clone + Serialize + for<'de> Deserialize<'de>;

    type Error: std::error::Error + Send + Sync;

    type TaskError: std::error::Error + Send + Sync;

    #[allow(unused_variables)]
    fn install(&self, ctx: &mut ContextBuilder<'ctx, Self>) {}

    async fn init(
        &self,
        ctx: &'ctx Context<'ctx, Self>,
    ) -> Result<TaskGraph<Self::Task>, Self::Error>;

    async fn run_task(
        &self,
        ctx: TaskContext<'ctx, Self>,
        task: &Self::TaskId,
    ) -> Result<bool, Self::Error>;

    async fn abort_task(
        &self,
        ctx: TaskContext<'ctx, Self>,
        task: &Self::TaskId,
    ) -> Result<TaskStatus, Self::Error>;

    async fn abort_all_tasks(&self, ctx: &'ctx Context<'ctx, Self>) -> Result<(), Self::Error>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum EngineStatus {
    #[default]
    Idle,
    Init,
    Active,
}
