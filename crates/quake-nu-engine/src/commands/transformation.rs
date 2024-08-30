use quake_core::metadata::Transform;

use super::prelude::*;

#[derive(Clone)]
pub struct Transforms;

impl Command for Transforms {
    fn name(&self) -> &str {
        "transforms"
    }

    fn signature(&self) -> Signature {
        Signature::build("transforms")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required(
                "sources",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "source files",
            )
            .required(
                "artifacts",
                SyntaxShape::Keyword(
                    b"into".to_vec(),
                    Box::new(SyntaxShape::List(Box::new(SyntaxShape::String))),
                ),
                "artifacts produced by the given source files",
            )
            .optional(
                "run_body",
                SyntaxShape::Keyword(b"with".to_vec(), Box::new(SyntaxShape::Closure(None))),
                "optional body to define transformation behavior",
            )
            .category(Category::Custom(QUAKE_CATEGORY.to_owned()))
    }

    fn usage(&self) -> &str {
        "Declare source-to-artifact transformations performed by this task"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let sources: Vec<String> = call.req(engine_state, stack, 0)?;
        let artifacts: Vec<String> = call.req(engine_state, stack, 1)?;
        let run_body = call
            .opt::<Closure>(engine_state, stack, 2)?
            .map(|c| c.block_id);

        State::capture_errors_in_shell(engine_state, |state| {
            state
                .scope_metadata_mut(stack, call.head)?
                .push_transform(Transform {
                    sources,
                    artifacts,
                    run_body,
                });
            Ok(())
        })?;

        Ok(PipelineData::empty())
    }
}

#[derive(Clone)]
pub struct Sources;

impl Command for Sources {
    fn name(&self) -> &str {
        "sources"
    }

    fn signature(&self) -> Signature {
        Signature::build("sources")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required(
                "files",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "files to be included as sources",
            )
            .category(Category::Custom(QUAKE_CATEGORY.to_owned()))
    }

    fn usage(&self) -> &str {
        "Declare files this task depends upon as sources"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let values: Vec<String> = call.req(engine_state, stack, 0)?;

        State::capture_errors_in_shell(engine_state, |state| {
            state
                .scope_metadata_mut(stack, call.head)?
                .extend_sources(values);
            Ok(())
        })?;

        Ok(PipelineData::empty())
    }
}

#[derive(Clone)]
pub struct Artifacts;

impl Command for Artifacts {
    fn name(&self) -> &str {
        "artifacts"
    }

    fn signature(&self) -> Signature {
        Signature::build("artifacts")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required(
                "files",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "files to be produced",
            )
            .category(Category::Custom(QUAKE_CATEGORY.to_owned()))
    }

    fn usage(&self) -> &str {
        "Declare files to be produced by a task"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let values: Vec<String> = call.req(engine_state, stack, 0)?;

        State::capture_errors_in_shell(engine_state, |state| {
            state
                .scope_metadata_mut(stack, call.head)?
                .extend_artifacts(values);
            Ok(())
        })?;

        Ok(PipelineData::empty())
    }
}
