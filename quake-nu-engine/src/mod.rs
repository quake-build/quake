use std::path::Path;
use std::sync::Arc;

use nu_cli::gather_parent_env_vars;
use nu_cmd_lang::create_default_context;
use nu_command::add_shell_command_context;
use nu_protocol::engine::{EngineState, Stack, StateWorkingSet, PWD_ENV};
use nu_protocol::{Span, Type, Value, VarId};
use parking_lot::RwLock;

use crate::state::State;

pub mod commands;
pub mod eval;
pub mod parse;
pub mod types;
pub mod utils;
