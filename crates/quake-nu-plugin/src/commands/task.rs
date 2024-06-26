use super::prelude::*;
use super::CategoryExt;
use crate::QuakePlugin;

pub struct TaskCommand;

impl PluginCommand for TaskCommand {
    type Plugin = QuakePlugin;

    fn name(&self) -> &str {
        "task"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("task")
            .required("name", SyntaxShape::String, "task name")
            .required("body", SyntaxShape::Any, "task body")
            .category(Category::quake())
    }

    fn usage(&self) -> &str {
        ""
    }

    fn extra_usage(&self) -> &str {
        ""
    }

    fn search_terms(&self) -> Vec<&str> {
        vec![]
    }

    fn examples(&self) -> Vec<Example> {
        vec![]
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        Ok(PipelineData::empty())
    }
}
