use quake_core::metadata::TaskCallId;

use super::prelude::*;

#[derive(Clone)]
pub struct Dependency;

impl Command for Dependency {
    fn name(&self) -> &str {
        "dependency"
    }

    fn signature(&self) -> Signature {
        Signature::build("dependency")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required("dep_id", SyntaxShape::String, "dependency ID")
            .allows_unknown_args()
            .category(Category::Custom(QUAKE_CATEGORY.to_owned()))
    }

    fn usage(&self) -> &str {
        "Depend on another quake task"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        // (parser internal, replaced with `ResolvedDependency`)

        // emit an error if used in an invalid location
        State::capture_errors_in_shell(engine_state, |state| {
            state.check_in_scope(stack, call.head)
        })?;

        Ok(PipelineData::empty())
    }
}

#[derive(Clone)]
pub struct ResolvedDependency {
    pub task_id: TaskCallId,
    pub signature: Box<Signature>,
}

impl Command for ResolvedDependency {
    fn name(&self) -> &str {
        &self.signature.name
    }

    fn signature(&self) -> Signature {
        *self.signature.clone()
    }

    fn usage(&self) -> &str {
        &self.signature.usage
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        State::capture_errors_in_shell(engine_state, |state| {
            state.check_in_scope(stack, call.head)?;

            // register the call_id and add it as a dependency
            let call_id = state
                .metadata
                .register_task_call(
                    self.task_id,
                    call.span(),
                    call.arguments.clone(),
                    Vec::new(),
                )
                .unwrap();
            state
                .scope_metadata_mut(stack, call.head)?
                .push_dep(call_id);

            Ok(())
        })?;

        Ok(PipelineData::Empty)
    }
}
