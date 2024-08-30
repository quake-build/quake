use super::prelude::*;

#[derive(Clone)]
pub struct DefTask;

impl Command for DefTask {
    fn name(&self) -> &str {
        "def-task"
    }

    fn usage(&self) -> &str {
        "Define a quake task."
    }

    fn extra_usage(&self) -> &str {
        "Must provide a declaration body (prefixed with 'where') and/or a run body (prefixed with \
         'do')."
    }

    fn signature(&self) -> Signature {
        Signature::build("def-task")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required("name", SyntaxShape::String, "task name")
            .required("params", SyntaxShape::Signature, "parameters")
            .switch(
                "concurrent",
                "allow this task to be run concurrently with others",
                Some('c'),
            )
            .optional(
                "decl_body",
                SyntaxShape::Keyword(b"where".to_vec(), Box::new(SyntaxShape::Closure(None))),
                "body to be evaluated when this task is triggered",
            )
            .optional(
                "run_body",
                SyntaxShape::Keyword(b"do".to_vec(), Box::new(SyntaxShape::Closure(None))),
                "body to be evaluated when this task is triggered",
            )
            .creates_scope()
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        // (parser internal)
        Ok(PipelineData::Empty)
    }

    fn run_const(
        &self,
        _working_set: &StateWorkingSet,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        // (parser internal)
        Ok(PipelineData::Empty)
    }

    fn is_const(&self) -> bool {
        true
    }
}
