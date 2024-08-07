use nu_protocol::ast::{Argument, Block};
use nu_protocol::debugger::WithoutDebug;
use nu_protocol::engine::{EngineState, Stack};
use nu_protocol::{PipelineData, Span, Value, VarId};

use quake_core::metadata::TaskCallId;
use quake_core::prelude::*;

use crate::state::State;
use crate::utils::set_last_exit_code;

pub fn eval_block(
    block: &Block,
    engine_state: &EngineState,
    stack: &mut Stack,
) -> ShellResult<bool> {
    if block.is_empty() {
        return Ok(true);
    }

    let result =
        nu_engine::eval_block::<WithoutDebug>(engine_state, stack, block, PipelineData::Empty);

    // reset vt processing, aka ansi because illbehaved externals can break it
    #[cfg(windows)]
    {
        let _ = nu_utils::enable_vt_processing();
    }

    match result {
        Ok(pipeline_data) => {
            let exit_code = pipeline_data.print(engine_state, stack, false, false)?;
            set_last_exit_code(stack, exit_code);
            if exit_code != 0 {
                return Ok(false);
            }
        }
        Err(err) => {
            set_last_exit_code(stack, 1);
            return Err(err);
        }
    }

    Ok(true)
}

pub fn eval_task_decl_body(
    call_id: TaskCallId,
    engine_state: &EngineState,
    stack: &mut Stack,
) -> ShellResult<bool> {
    // convert task stub into task metadata
    let (call, decl_body) = {
        let state = State::from_engine_state(engine_state);
        let call = state.metadata.get_task_call(call_id).unwrap().clone();
        let task = state.metadata.get_task(call.task_id).unwrap();

        let Some(decl_body) = task.decl_body else {
            // no decl body: early return with no additional metadata
            return Ok(true);
        };

        (call, decl_body)
    };

    // push task scope (will error if nested inside another task body)
    State::capture_errors_in_shell(engine_state, |state| {
        state.push_scope(call_id, stack, call.span)
    })?;

    // evaluate declaration body
    let block = engine_state.get_block(decl_body);
    let success = eval_body(
        block,
        &call.arguments,
        &call.constants,
        call.span,
        engine_state,
        stack,
    )?;

    // pop task scope
    State::capture_errors_in_shell(engine_state, |state| state.pop_scope(stack, call.span))?;

    Ok(success)
}

pub fn eval_task_run_body(
    call_id: TaskCallId,
    span: Span,
    engine_state: &EngineState,
    stack: &mut Stack,
) -> ShellResult<bool> {
    // fetch metadata
    let (block_id, call) = {
        let state = State::from_engine_state(engine_state);

        let call = state.metadata.get_task_call(call_id).unwrap();
        let block_id = state.metadata.get_task(call.task_id).unwrap().run_body;

        if block_id.is_none() {
            return Ok(true);
        }

        (block_id.unwrap(), call)
    };

    // evaluate run body (no call scope added)
    let block = engine_state.get_block(block_id);
    eval_body(
        block,
        &call.arguments,
        &call.constants,
        span,
        engine_state,
        stack,
    )
}

/// Similar to [`eval_call`](nu_engine::eval_call), but with manual blocks and
/// arguments.
fn eval_body(
    block: &Block,
    arguments: &[Argument],
    constants: &[(VarId, Value)],
    span: Span,
    engine_state: &EngineState,
    stack: &mut Stack,
) -> ShellResult<bool> {
    let signature = &block.signature;

    let mut callee_stack = stack.gather_captures(engine_state, &block.captures);

    let mut positional_arg_vals = Vec::with_capacity(arguments.len());
    let mut named_arg_vals = Vec::with_capacity(arguments.len());
    let mut rest_arg_val = None;

    for arg in arguments {
        match arg {
            Argument::Positional(a) => positional_arg_vals.push(a),
            Argument::Named(a) => named_arg_vals.push(a),
            Argument::Spread(rest) => rest_arg_val = Some(rest),
            Argument::Unknown(_) => unimplemented!("Argument::Unknown in task call"),
        }
    }

    for (param_idx, param) in signature
        .required_positional
        .iter()
        .chain(signature.optional_positional.iter())
        .enumerate()
    {
        let value = if let Some(expr) = positional_arg_vals.get(param_idx) {
            nu_engine::eval_expression::<WithoutDebug>(engine_state, stack, expr)?
        } else if let Some(value) = &param.default_value {
            value.clone()
        } else {
            Value::nothing(span)
        };

        callee_stack.add_var(param.var_id.unwrap(), value);
    }

    if let (Some(rest_arg), Some(rest_val)) = (&signature.rest_positional, &rest_arg_val) {
        callee_stack.add_var(
            rest_arg.var_id.unwrap(),
            nu_engine::eval_expression::<WithoutDebug>(engine_state, stack, rest_val)?,
        );
    }

    for named in &signature.named {
        let var_id = named.var_id.unwrap();

        let value = if let Some(expr) = named_arg_vals
            .iter()
            .find(|(long, short, _)| {
                named.long == long.item
                    || (named.short.is_some()
                        && named.short == short.as_ref().and_then(|s| s.item.chars().next()))
            })
            .map(|(_, _, expr)| expr)
        {
            if let Some(expr) = expr {
                nu_engine::eval_expression::<WithoutDebug>(engine_state, stack, expr)?
            } else if let Some(value) = &named.default_value {
                value.clone()
            } else {
                Value::bool(true, span)
            }
        } else if named.arg.is_none() {
            Value::bool(false, span)
        } else if let Some(value) = &named.default_value {
            value.clone()
        } else {
            Value::nothing(span)
        };

        callee_stack.add_var(var_id, value);
    }

    for (var_id, value) in constants {
        callee_stack.add_var(*var_id, value.clone());
    }

    eval_block(block, engine_state, &mut callee_stack)
}
