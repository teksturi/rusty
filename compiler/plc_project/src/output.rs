use serde::{Serialize, Deserialize};

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

