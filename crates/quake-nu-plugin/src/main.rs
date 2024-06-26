use nu_plugin::{serve_plugin, MsgPackSerializer};
use quake_nu_plugin::QuakePlugin;

fn main() {
    serve_plugin(&QuakePlugin::new(), MsgPackSerializer);
}
