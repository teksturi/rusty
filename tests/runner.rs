use driver::pipelines::ParsedProject;
use inkwell::{
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
};
use project::project::Project;
use rusty::{lexer::IdProvider, diagnostics::Diagnostician, codegen::{CodegenContext, GeneratedModule}};
use source::{SourceContainer, SourceCode};


#[allow(dead_code)]
#[repr(C)]
pub struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

pub trait Compilable {
    type T: SourceContainer;
    fn containers(self) -> Vec<Self::T>;
}

impl Compilable for &str {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = Self::T::from(self);
        vec![code]
    }
}

impl Compilable for String {
    type T = SourceCode;
    fn containers(self) -> Vec<Self::T> {
        let code = self.into();
        vec![code]
    }
}

impl<S: SourceContainer> Compilable for Vec<S> {
    type T = S;
    fn containers(self) -> Vec<Self::T> {
        self
    }
}

impl Compilable for SourceCode {
    type T = Self;

    fn containers(self) -> Vec<Self::T> {
        vec![self]
    }
}

///
/// Compiles and runs the given sources
/// Sources must be `Compilable`, default implementations include `String` and `&str`
/// An implementation is also provided for `Vec<SourceContainer>`
///
pub fn compile<'ink, T: Compilable>(context: &'ink CodegenContext, source: T) -> GeneratedModule<'ink> {
    let source = source.containers();
    let project = Project::new("TestProject".to_string()).with_sources(source);
    let mut diagnostician = Diagnostician::null_diagnostician();
    let id_provider = IdProvider::default();
    let parsed_project = ParsedProject::parse(project, None, id_provider.clone(), &mut diagnostician).unwrap();
    let indexed_project = parsed_project.index(id_provider.clone()).unwrap();
    let annotated_project = indexed_project.annotate(id_provider.clone(), &diagnostician).unwrap();
    let module = annotated_project.codegen_to_single_module(context, None, rusty::OptimizationLevel::None, rusty::DebugLevel::None).unwrap().unwrap();

    #[cfg(feature = "debug")]
    module.print_to_stderr();

    module

}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: CodegenContext = CodegenContext::new();
    let module = compile(&context, source);
    module.run::<T, U>("main", params)
}
