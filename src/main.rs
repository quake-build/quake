#![feature(stmt_expr_attributes)]

// TODO consider deriving from Parser instead
fn parse_args() -> clap::ArgMatches {
    use clap::builder::*;
    use clap::*;

    Command::new("quake")
        .about("quake: a meta-build system powered by nushell")
        .version(crate_version!())
        .color(ColorChoice::Never)
        .max_term_width(100)
        .override_usage(
            #[rustfmt::skip]
            "quake [OPTIONS] <TASK> [--] [TASK_ARGS]\n       \
             quake [OPTIONS]",
        )
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .args_conflicts_with_subcommands(true)
        .subcommand_negates_reqs(true)
        .subcommand_help_heading("Subcommands")
        .subcommands([
            Command::new("list").about("List the available tasks"),
            Command::new("inspect").about("Dump build script metadata as JSON"),
        ])
        .next_help_heading("Environment")
        .args([Arg::new("project")
            .long("project")
            .value_name("PROJECT_DIR")
            .value_parser(PathBufValueParser::new())
            .value_hint(ValueHint::DirPath)
            .help("Path to the project root directory")
            .global(true)])
        .next_help_heading("Output handling")
        .args([
            Arg::new("quiet")
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress the output (stdout and stderr) of any executed commands"),
            Arg::new("json")
                .long("json")
                .action(ArgAction::SetTrue)
                .help(
                    "Output events as a line-delimited JSON objects to stderr. See the JSON \
                     appendix in the manual for the specification of these objects.",
                )
                .global(true),
        ])
        .next_help_heading("Evaluation modes")
        .args([
            Arg::new("force")
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Execute tasks regardless of initial dirtiness checks"),
            Arg::new("watch")
                .long("watch")
                .action(ArgAction::SetTrue)
                .help("Run the task, and re-run whenever sources have changed"),
        ])
        .args([
            Arg::new("task").value_name("TASK").hide(true),
            Arg::new("task-args")
                .value_name("TASK_ARGS")
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(0..)
                .hide(true),
        ])
        .get_matches()
}

fn main() {
    let args = parse_args();

    // TODO contruct Runtime via RuntimeBuilder, with E = NuEngine
    // (and handle all of the other flags)
}
