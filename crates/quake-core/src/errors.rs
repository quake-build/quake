use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("Project not found")]
#[diagnostic(code(quake::project_not_found))]
pub struct ProjectNotFound;

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("Unable to locate build script in project")]
#[diagnostic(
    code(quake::build_script_not_found),
    help("Add a `build.quake` file to the project root")
)]
pub struct BuildScriptNotFound;
