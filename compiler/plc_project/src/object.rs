use std::path::{Path, PathBuf};

use diagnostics::Diagnostic;

/// Representation of a binary file
#[derive(Debug)]
pub enum Object {
    /// Archive file containing several object files, used for static linking
    Archive(PathBuf),
    /// Shared object or DLL, used to link to other objects
    Shared(PathBuf),
    /// An executable file
    Executable(PathBuf),
    /// An LLVM Bitcode generated file (".bc")
    Bitcode(PathBuf),
    /// An LLVM IR generated file (".ll")
    IR(PathBuf),
    /// Default non specific representation, this is typically the ".o" file
    Default(PathBuf),
}

impl TryFrom<&Path> for Object {
    type Error = Diagnostic;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        match value.extension().and_then(|it| it.to_str()) {
            Some("o") => Ok(Object::Default(value.to_path_buf())),
            Some("bc") => Ok(Object::Bitcode(value.to_path_buf())),
            Some("ir") => Ok(Object::IR(value.to_path_buf())),
            Some("so") => Ok(Object::Shared(value.to_path_buf())),
            Some("a") => Ok(Object::Archive(value.to_path_buf())),
            None => Ok(Object::Executable(value.to_path_buf())),
            Some(any) => Err(Diagnostic::GeneralError {
                message: format!(
                    "Could derive object type for {}: Unknown extension {any}",
                    value.to_string_lossy()
                ),
                err_no: diagnostics::ErrNo::general__io_err,
            }),
        }
    }
}
