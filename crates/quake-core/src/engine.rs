use std::error::Error as StdError;
use std::fmt::Debug;

use async_trait::async_trait;

use crate::context::EngineCtx;
use crate::error::Severity;
use crate::task::TaskStatus;

#[async_trait]
pub trait Engine: Sized {
    type Task: Send + Sync;
    type Event: Debug + Send + Sync;
    type Error: EngineError;

    async fn init(&self, ctx: &EngineCtx<Self>) -> Result<(), Self::Error>;

    async fn run_task(
        &self,
        ctx: &EngineCtx<Self>,
        task: &Self::TaskId,
    ) -> Result<bool, Self::Error>;

    fn abort_task(
        &self,
        ctx: &EngineCtx<Self>,
        task: &Self::TaskId,
    ) -> Result<TaskStatus, Self::Error>;

    fn abort_all_tasks(&self, ctx: &EngineCtx<Self>) -> Result<(), Self::Error>;
}

pub trait EngineError: StdError + Send + Sync {
    fn severity(&self) -> Severity;

    fn is_fatal(&self) -> bool;
}
