// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
//!A St&ructured Text LLVM Frontent
//!
//! RuSTy is an [`ST`] Compiler using LLVM
//!
//! # Features
//! ## Standard language support
//! Most of the [`IEC61131-3`] standard for ST and general programing is supported.
//! ## Native compilation
//! A (currently) single ST files into object code using LLVM.
//! A compiled object can be linked statically or dynamically linked
//!     with other programs using standard compiler linkers (ld, clang, gcc)
//! ## IR Output
//! An [`IR`] file can be generated from any given ST file in order to examin the generated LLVM IR code.
//! For a usage guide refer to the [User Documentation](../../)
//!
//! [`ST`]: https://en.wikipedia.org/wiki/Structured_text
//! [`IEC61131-3`]: https://en.wikipedia.org/wiki/IEC_61131-3
//! [`IR`]: https://llvm.org/docs/LangRef.html
use std::convert::Infallible;
use std::str::FromStr;

use clap::clap_derive::ArgEnum;
use codegen::{CodeGen, CodegenContext};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use diagnostics::Diagnostic;
use index::Index;
use inkwell::targets::{
    self, TargetMachine, TargetTriple,
};
use resolver::{AstAnnotations, StringLiterals};

#[cfg(test)]
use resolver::TypeAnnotator;
#[cfg(test)]
use validation::Validator;

use crate::ast::CompilationUnit;

pub mod ast;
pub mod builtins;
pub mod codegen;
mod datalayout;
pub mod diagnostics;
pub mod expression_path;
mod hardware_binding;
pub mod index;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod linker;
mod test_utils;

pub mod typesystem;
pub mod validation;

#[cfg(test)]
mod tests {
    mod adr;
}

#[macro_use]
#[cfg(test)]
extern crate pretty_assertions;

extern crate shell_words;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    System,
    Param { triple: String, sysroot: Option<String> },
}

impl Target {
    pub fn new(triple: String, sysroot: Option<String>) -> Target {
        Target::Param { triple, sysroot }
    }

    pub fn with_sysroot(self, sysroot: Option<String>) -> Target {
        match self {
            Self::Param {triple, .. } => Target::Param { triple , sysroot },
            _ => self
        }
    }

    pub fn get_target_triple(&self) -> TargetTriple {
        let res = match self {
            Target::System => TargetMachine::get_default_triple(),
            Target::Param { triple, .. } => TargetTriple::create(triple),
        };
        targets::TargetMachine::normalize_triple(&res)
    }

    pub fn try_get_name(&self) -> Option<&str> {
        match self {
            Target::System => None,
            Target::Param { triple, .. } => Some(triple.as_str()),
        }
    }

    pub fn get_sysroot(&self) -> Option<&str> {
        match self {
            Target::Param { sysroot, .. } => sysroot.as_deref(),
            _ => None,
        }
    }
}

impl<T> From<T> for Target
where
    T: core::ops::Deref<Target = str>,
{
    fn from(it: T) -> Self {
        Target::new(it.to_string(), None)
    }
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Target::from(s))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FormatOption {
    /// Indicates that the result will be an object file (e.g. No Linking)
    Object,
    /// Indicates that the output format will be linked statically (i.e. Executable)
    Static,
    /// Indicates that the linked object will be shared and position independent
    Shared,
    /// Indicates that the compiled object will be relocatable (e.g. Combinable into multiple objects)
    Relocatable,
    /// Indicates that the compile result will be LLVM Bitcode
    Bitcode,
    /// Indicates that the compile result will be LLVM IR
    IR,
}

impl Default for FormatOption {
    fn default() -> Self {
        FormatOption::Object
    }
}



impl FormatOption {
    pub fn should_link(self) -> bool {
        matches!(
            self,
            FormatOption::Static | FormatOption::Shared | FormatOption::Relocatable
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, ArgEnum)]
pub enum ConfigFormat {
    JSON,
    TOML,
}

impl FromStr for ConfigFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ConfigFormat::JSON),
            "toml" => Ok(ConfigFormat::TOML),
            _ => Err(format!("Invalid option {s}")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompileOptions {
    /// Default project location (where the plc.json is defined, or where we are currently
    /// compiling)
    pub root: Option<PathBuf>,
    /// The location where the build would happen. This is None if the build subcommand was not
    /// used
    pub build_location: Option<PathBuf>,
    /// The name of the resulting compiled file
    pub output: String,
    pub optimization: OptimizationLevel,
    pub error_format: ErrorFormat,
    pub debug_level: DebugLevel,
}

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            root: None,
            build_location: None,
            output: String::new(),
            optimization: OptimizationLevel::None,
            error_format: ErrorFormat::None,
            debug_level: DebugLevel::None,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct LinkOptions {
    pub libraries: Vec<String>,
    pub library_pathes: Vec<String>,
    pub format: FormatOption,
    pub linker: Option<String>,
}

#[derive(Clone)]
pub struct ConfigurationOptions {
    format: ConfigFormat,
    output: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ArgEnum, Serialize, Deserialize)]
pub enum ErrorFormat {
    Rich,
    Clang,
    None,
}

impl Default for ErrorFormat {
    fn default() -> Self {
        ErrorFormat::Rich
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Threads {
    Full,
    Fix(i32),
    None,
}

impl Default for Threads {
    fn default() -> Self {
        Threads::None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugLevel {
    None,
    VariablesOnly,
    Full,
}

impl Default for DebugLevel {
    fn default() -> Self {
        Self::None
    }
}

impl From<OptimizationLevel> for inkwell::OptimizationLevel {
    fn from(val: OptimizationLevel) -> Self {
        match val {
            OptimizationLevel::None => inkwell::OptimizationLevel::None,
            OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }
}

impl OptimizationLevel {
    fn opt_params(&self) -> &str {
        match self {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        }
    }

    fn is_optimized(&self) -> bool {
        !matches!(self, OptimizationLevel::None)
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}
