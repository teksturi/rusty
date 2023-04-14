#[cfg(test)]
pub mod tests {

    use std::{cell::RefCell, path::PathBuf, rc::Rc, str::FromStr};

    use inkwell::context::Context;

    use crate::{
        ast::{self, CompilationUnit, SourceRangeFactory},
        builtins,
        codegen::CodegenContext,
        diagnostics::{Diagnostic, DiagnosticReporter, Diagnostician, ResolvedDiagnostics},
        index::{self, Index},
        lexer::{self, IdProvider},
        parser,
        resolver::{const_evaluator::evaluate_constants, AnnotationMapImpl, AstAnnotations, TypeAnnotator},
        typesystem::get_builtin_types,
        CompileOptions, DebugLevel, Validator,
    };

    ///a Diagnostic reporter that holds all diagnostics in a list
    #[derive(Default)]
    #[cfg(test)]
    pub struct ListBasedDiagnosticReporter {
        last_id: usize,
        // RC to access from tests, RefCell to avoid changing the signature for the report() method
        diagnostics: Rc<RefCell<Vec<ResolvedDiagnostics>>>,
    }

    #[cfg(test)]
    impl DiagnosticReporter for ListBasedDiagnosticReporter {
        fn report(&self, diagnostics: &[ResolvedDiagnostics]) {
            self.diagnostics.borrow_mut().extend_from_slice(diagnostics);
        }

        fn register(&mut self, _path: String, _src: String) -> usize {
            // at least provide some unique ids
            self.last_id += 1;
            self.last_id
        }
    }

    /// creates a diagnostician that just saves passed diagnostics, it is mainly used in tests
    #[cfg(test)]
    pub fn list_based_diagnostician(diagnostics: Rc<RefCell<Vec<ResolvedDiagnostics>>>) -> Diagnostician {
        use std::collections::HashMap;

        use crate::diagnostics::DefaultDiagnosticAssessor;

        Diagnostician {
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            reporter: Box::new(ListBasedDiagnosticReporter { diagnostics, ..Default::default() }),
            filename_fileid_mapping: HashMap::new(),
        }
    }

    pub fn parse(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        parser::parse(
            lexer::lex_with_ids(src, IdProvider::default(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            "test.st",
        )
    }

    pub fn parse_and_preprocess(src: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        let id_provider = IdProvider::default();
        let (mut unit, diagnostic) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            "test.st",
        );
        ast::pre_process(&mut unit, id_provider);
        (unit, diagnostic)
    }

    fn do_index(src: &str, id_provider: IdProvider) -> (CompilationUnit, Index) {
        let mut index = Index::default();
        //Import builtins
        let builtins = builtins::parse_built_ins(id_provider.clone());

        index.import(index::visitor::visit(&builtins));
        // import built-in types like INT, BOOL, etc.
        for data_type in get_builtin_types() {
            index.register_type(data_type);
        }

        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            "test.st",
        );
        ast::pre_process(&mut unit, id_provider);
        index.import(index::visitor::visit(&unit));
        (unit, index)
    }

    pub fn index(src: &str) -> (CompilationUnit, Index) {
        let id_provider = IdProvider::default();
        do_index(src, id_provider)
    }

    pub fn index_with_ids(src: &str, id_provider: IdProvider) -> (CompilationUnit, Index) {
        do_index(src, id_provider)
    }

    pub fn annotate_with_ids(
        parse_result: &CompilationUnit,
        index: &mut Index,
        id_provider: IdProvider,
    ) -> AnnotationMapImpl {
        let (mut annotations, _) = TypeAnnotator::visit_unit(index, parse_result, id_provider);
        index.import(std::mem::take(&mut annotations.new_index));
        annotations
    }

    pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
        let id_provider = IdProvider::default();
        let (unit, index) = index_with_ids(src, id_provider.clone());

        let (mut index, ..) = evaluate_constants(index);
        let (mut annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
        index.import(std::mem::take(&mut annotations.new_index));

        let mut validator = Validator::new();
        validator.perform_global_validation(&index);
        validator.visit_unit(&annotations, &index, &unit);
        validator.diagnostics()
    }

    pub fn codegen_without_unwrap(src: &str) -> Result<String, Diagnostic> {
        codegen_debug_without_unwrap(src, DebugLevel::None)
    }

    /// Returns either a string or an error, in addition it always returns
    /// reported diagnostics. Therefor the return value of this method is always a tuple.
    /// TODO: This should not be so, we should have a diagnostic type that holds multiple new
    /// issues.
    pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, Diagnostic> {
        let mut id_provider = IdProvider::default();
        let (unit, index) = do_index(src, id_provider.clone());

        let (mut index, ..) = evaluate_constants(index);
        let (mut annotations, literals) = TypeAnnotator::visit_unit(&index, &unit, id_provider.clone());
        index.import(std::mem::take(&mut annotations.new_index));

        let context = CodegenContext::new();
        let path = PathBuf::from_str("src").ok();
        let mut code_generator = crate::codegen::CodeGen::new(
            &context,
            path.as_deref(),
            "main",
            crate::OptimizationLevel::None,
            debug_level,
        );
        let annotations = AstAnnotations::new(annotations, id_provider.next_id());
        let llvm_index = code_generator.generate_llvm_index(&context, &annotations, &literals, &index)?;

        code_generator
            .generate(&context, &unit, &annotations, &index, &llvm_index)
            .map(|module| module.persist_to_string())
    }

    pub fn codegen_with_debug(src: &str) -> String {
        codegen_debug_without_unwrap(src, DebugLevel::Full).unwrap()
    }

    pub fn codegen(src: &str) -> String {
        codegen_without_unwrap(src).unwrap()
    }

    pub fn generate_with_empty_program(src: &str) -> String {
        let source = format!("{} {}", "PROGRAM main END_PROGRAM", src);
        codegen(source.as_str())
    }
}
