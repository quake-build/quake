#[allow(unused_imports)]
mod prelude {
    pub use nu_engine::CallExt;
    pub use nu_protocol::ast::Call;
    pub use nu_protocol::engine::{Closure, Command, EngineState, Stack, StateWorkingSet};
    pub use nu_protocol::{BlockId, Category, Example, PipelineData, Signature, SyntaxShape, Type};

    pub use quake_core::prelude::*;

    pub use crate::state::State;
    pub use crate::QUAKE_CATEGORY;
}

mod def_task;
mod dependency;
mod transformation;

pub use def_task::DefTask;
pub use dependency::{Dependency, ResolvedDependency};
pub use transformation::{Artifacts, Sources, Transforms};
