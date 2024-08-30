use crate::engine::Engine;
use crate::task::TaskGraph;

#[derive(PartialEq, Eq)]
pub struct Metadata<E: Engine> {
    pub task_graph: TaskGraph<E::Task>,
}

impl<E: Engine> Metadata<E> {
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<E: Engine> Default for Metadata<E> {
    fn default() -> Self {
        Self {
            task_graph: Default::default(),
        }
    }
}
