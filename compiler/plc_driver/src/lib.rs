//! Compiler driver for the PLC Compiler
//!
//! This crates offers the main methods to interact with the PLC Compiler
//! It can be used to verify a project or to produce:
//!  - Object files
//!  - LLVM files
//!  - LLVM Bitcode
//!  - Shared Objects
//!  - Executables

use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use cli::CompileParameters;
use diagnostics::{Diagnostic, Diagnostician};
use plc::lexer::IdProvider;
use project::project::Project;
use source_code::SourceContainer;

pub mod cli;
pub mod pipelines;

#[cfg(test)]
mod tests;

pub(crate) const DEFAULT_OUTPUT_NAME: &str = "out";

pub fn compile<T: AsRef<str> + AsRef<OsStr>>(args: &[T]) -> Result<(), Diagnostic> {
    //Parse the arguments
    let compile_parameters = CompileParameters::parse(args)?;
    let project = get_project(&compile_parameters)?;
    let location = project.get_location().map(|it| it.to_path_buf());
    let id_provider = IdProvider::default();
    let mut diagnostician = Diagnostician::default(); //TODO
                                                      // 1 : Parse
    let annotated_project = pipelines::ParsedProject::parse(
        project,
        compile_parameters.encoding,
        id_provider.clone(),
        &mut diagnostician,
    )?
    // 2 : Index
    .index(id_provider.clone())?
    // 3 : Resolve
    .annotate(id_provider.clone(), &diagnostician)?;
    // 4 : Validate and Codegen (parallel)
    annotated_project.validate(&diagnostician)?;
    let res = annotated_project.codegen(
        location.as_deref(),
        compile_parameters.get_build_location().as_deref(),
        compile_parameters.optimization,
        compile_parameters.debug_level(),
        compile_parameters.output_format_or_default(),
        &compile_parameters.target,
    )?;
    // 5 : Link
    res.link(todo!()/*link_options*/)?;
    Ok(())
}

fn get_project(compile_parameters: &CompileParameters) -> Result<Project<PathBuf>, Diagnostic> {
    let current_dir = env::current_dir()?;
    //Create a project from either the subcommand or single params
    if let Some(command) = &compile_parameters.commands {
        //Build with subcommand
        let config = command
            .get_build_configuration()
            .map(|it| PathBuf::from(it))
            .or_else(|| get_config(&current_dir))
            .ok_or_else(|| Diagnostic::param_error("Could not find 'plc.json'"))?;
        Project::from_config(&config)
    } else {
        //Build with parameters
        let name = compile_parameters
            .input
            .get(0)
            .and_then(|it| it.get_location().file_name())
            .and_then(|it| it.to_str()).unwrap_or(DEFAULT_OUTPUT_NAME);
        let project = Project::new(name.to_string())
            .with_file_pathes(compile_parameters.input.iter().map(PathBuf::from).collect())
            .with_include_pathes(compile_parameters.includes.iter().map(PathBuf::from).collect())
            .with_libraries(compile_parameters.libraries.clone());
        Ok(project)
    }
}

fn get_config(root: &Path) -> Option<PathBuf> {
    Some(root.join("plc.json"))
}

pub fn get_output_name(
    as_deref: Option<&str>,
    output_format_or_default: plc::FormatOption,
    input: &str,
) -> String {
    todo!()
}
