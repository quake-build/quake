use crate::tasks::TaskGraph;

pub struct Metadata<T> {
    pub task_graph: TaskGraph<T>,
}

impl<T> Metadata<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Default for Metadata<T> {
    fn default() -> Self {
        Self {
            task_graph: Default::default(),
        }
    }
}
