use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::graph::Graph;

// TODO implement secondary "overlay" source/artifact hypergraph
// TODO after hypergraph, introduce glob set theoretic analysis for improved
// "magic" critical path analysis

pub type TaskId = String;

pub type TaskGraph<T> = Graph<TaskId, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Task<T> {
    pub id: TaskId,
    pub inner: T,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TaskStatus {
    #[default]
    Idle,
    Running,
    Aborted,
    Failure,
    Success,
}

impl TaskStatus {
    #[inline]
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Aborted | Self::Failure | Self::Success)
    }

    #[inline]
    pub const fn success(&self) -> Option<bool> {
        match self {
            Self::Success => Some(true),
            Self::Aborted | Self::Failure => Some(false),
            _ => None,
        }
    }
}
