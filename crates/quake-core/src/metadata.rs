use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::context::Context;

type TaskId = usize;

#[derive(Debug, Clone)]
pub struct Metadata {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct Task {
    /// The name or identifier of the task.
    pub name: String,
    pub sources: Vec<String>,
    pub artifacts: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TaskRef<'a> {
    id: TaskId,
    phantom: PhantomData<&'a ()>,
}

impl<'a> TaskRef<'a> {
    #[inline(always)]
    fn new(id: TaskId) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}
