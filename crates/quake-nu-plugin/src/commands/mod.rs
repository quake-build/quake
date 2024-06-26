pub mod task;
pub use self::task::*;

use nu_protocol::Category;

pub const QUAKE_CATEGORY: &str = "quake";

#[allow(unused_imports)]
mod prelude {
    pub use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
    pub use nu_protocol::{
        Category, Example, LabeledError, PipelineData, Signature, SyntaxShape, Type,
    };

    pub(self) use super::CategoryExt as _;
}

trait CategoryExt {
    #[inline(always)]
    fn quake() -> Category {
        Category::Custom(QUAKE_CATEGORY.to_owned())
    }
}

impl CategoryExt for Category {}
