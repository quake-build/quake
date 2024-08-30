#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::graph::Graph;

// TODO implement secondary "overlay" source/artifact hypergraph
// TODO after hypergraph, introduce glob set theoretic analysis for improved
// "magic" critical path analysis

// pub type TaskRef<> = ();
pub type TaskGraph<T> = Graph<String, T>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TaskStatus {
    #[default]
    Idle,
    Running,
    Terminated(TaskResult),
}

impl TaskStatus {
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Terminated(_))
    }

    pub const fn success(&self) -> Option<bool> {
        match self {
            Self::Terminated(TaskResult::Success) => Some(true),
            Self::Terminated(_) => None,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TaskResult {
    Failure,
    Aborted,
    Success,
}
