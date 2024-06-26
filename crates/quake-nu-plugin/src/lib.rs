pub(crate) mod commands;

use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

use crate::commands::*;

/// Nushell category name used for commands.
pub const QUAKE_CATEGORY: &str = "quake";

#[derive(Debug, Clone)]
pub struct QuakePlugin {}

impl QuakePlugin {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serve(&self) {
        serve_plugin(self, MsgPackSerializer)
    }
}

impl Plugin for QuakePlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(TaskCommand)]
    }
}
