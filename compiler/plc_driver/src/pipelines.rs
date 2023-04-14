use std::{path::{Path, PathBuf}, fs};

use ast::CompilationUnit;
use diagnostics::{Diagnostic, Diagnostician};
use encoding_rs::Encoding;
use plc::{
    codegen::{self, CodegenContext, GeneratedModule},
    index::Index,
    lexer::IdProvider,
    parser::parse_file,
    resolver::{AnnotationMap, AnnotationMapImpl, AstAnnotations, StringLiterals, TypeAnnotator},
    validation::Validator,
    CompileOptions, DebugLevel, FormatOption, LinkOptions, OptimizationLevel, Target,
};
use project::{
    object::Object,
    project::{LibraryInformation, Project},
};
use rayon::prelude::*;
use source_code::SourceContainer;
use tempfile::tempdir;

///Represents a parsed project
///For this struct to be built, the project would have been parsed correctly and an AST would have
///been generated
pub struct ParsedProject(Vec<CompilationUnit>);

impl ParsedProject {
    /// Parses a giving project, transforming it to a `ParsedProject`
    /// Reprots parsing diagnostics such as Syntax error on the fly
    pub fn parse<T: SourceContainer>(
        project: Project<T>,
        encoding: Option<&'static Encoding>,
        id_provider: IdProvider,
        diagnostician: &mut Diagnostician,
    ) -> Result<Self, Diagnostic> {
        //TODO in parallel
        //Parse the source files
        let mut units = vec![];

        let sources = project
            .get_sources()
            .iter()
            .map(|it| {
                let loaded_source = it
                    .load_source(encoding)
                    .map_err(|err| Diagnostic::io_read_error(&it.get_location().to_string_lossy(), &err))?;
                Ok(parse_file(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::Internal,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(sources);
        //Parse the includes
        let includes = project
            .get_includes()
            .iter()
            .map(|it| {
                let loaded_source = it
                    .load_source(encoding)
                    .map_err(|err| Diagnostic::io_read_error(&it.get_location().to_string_lossy(), &err))?;
                Ok(parse_file(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::External,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(includes);
        //For each lib, parse the includes
        let lib_includes = project
            .get_libraries()
            .iter()
            .flat_map(LibraryInformation::get_includes)
            .map(|it| {
                let loaded_source = it
                    .load_source(encoding)
                    .map_err(|err| Diagnostic::io_read_error(&it.get_location().to_string_lossy(), &err))?;
                Ok(parse_file(
                    &loaded_source.source,
                    loaded_source.get_location_str(),
                    ast::LinkageType::External,
                    id_provider.clone(),
                    diagnostician,
                ))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;
        units.extend(lib_includes);

        Ok(ParsedProject(units))
    }

    /// Creates an index out of a pased project. The index could then be used to query datatypes
    pub fn index(self, id_provider: IdProvider) -> Result<IndexedProject, Diagnostic> {
        let indexed_units = self
            .0
            .into_iter()
            .map(|mut unit| {
                //Preprocess
                ast::pre_process(&mut unit, id_provider.clone());
                //import to index
                let index = plc::index::visitor::visit(&unit);

                (index, unit)
            })
            .collect::<Vec<_>>();

        let mut global_index = Index::default();
        let mut units = vec![];
        for (index, unit) in indexed_units {
            units.push(unit);
            global_index.import(index);
        }

        // import built-in types like INT, BOOL, etc.
        for data_type in plc::typesystem::get_builtin_types() {
            global_index.register_type(data_type);
        }
        // import builtin functions
        let builtins = plc::builtins::parse_built_ins(id_provider.clone());
        global_index.import(plc::index::visitor::visit(&builtins));

        Ok(IndexedProject { units, index: global_index })
    }
}

///A project that has also been indexed
/// Units inside an index project could be resolved and annotated
pub struct IndexedProject {
    units: Vec<CompilationUnit>,
    index: Index,
}

impl IndexedProject {
    /// Creates annotations on the project in order to facilitate codegen and validation
    pub fn annotate(
        self,
        mut id_provider: IdProvider,
        _diagnostician: &Diagnostician,
    ) -> Result<AnnotatedProject, Diagnostic> {
        //Resolve constants
        //TODO: Not sure what we are currently doing with unresolvables
        let (mut full_index, _unresolvables) = plc::resolver::const_evaluator::evaluate_constants(self.index);
        //Create and call the annotator
        let mut annotated_units: Vec<CompilationUnit> = Vec::new();
        let mut all_annotations = AnnotationMapImpl::default();
        let mut all_literals = StringLiterals::default();

        let result = self
            .units
            .into_iter()
            .map(|unit| {
                let (annotation, literals) =
                    TypeAnnotator::visit_unit(&full_index, &unit, id_provider.clone());
                (unit, annotation, literals)
            })
            .collect::<Vec<_>>();

        for (unit, annotation, literals) in result {
            annotated_units.push(unit);
            all_annotations.import(annotation);
            all_literals.import(literals);
        }

        full_index.import(std::mem::take(&mut all_annotations.new_index));

        let annotations = AstAnnotations::new(all_annotations, id_provider.next_id());

        Ok(AnnotatedProject {
            units: annotated_units,
            index: full_index,
            annotations,
            literals: all_literals,
        })
    }
}

/// A project that has been annotated with information about different types and used units
pub struct AnnotatedProject {
    units: Vec<CompilationUnit>,
    index: Index,
    annotations: AstAnnotations,
    literals: StringLiterals,
}

impl AnnotatedProject {
    /// Validates the project, reports any new diagnostics on the fly
    pub fn validate(&self, diagnostician: &Diagnostician) -> Result<(), Diagnostic> {
        // perform global validation
        let mut validator = Validator::new();
        validator.perform_global_validation(&self.index);
        diagnostician.handle(validator.diagnostics());

        //Perform per unit validation
        self.units.iter().for_each(|unit| {
            // validate unit
            validator.visit_unit(&self.annotations, &self.index, &unit);
            // log errors
            diagnostician.handle(validator.diagnostics());
        });
        Ok(())
    }

    pub fn codegen_to_string(
        &self,
        root: Option<&Path>,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Result<Vec<String>, Diagnostic> {
        self.units.iter().map(|unit| {
            let context = CodegenContext::new();
            self.generate_module(&context, root, unit, optimization, debug_level).map(|it| it.persist_to_string())
        }).collect()
    }

    pub fn codegen_to_single_module<'ctx>(
        self,
        context: &'ctx CodegenContext,
        root: Option<&Path>,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Result<Option<GeneratedModule<'ctx>>, Diagnostic> {
        let Some(module) = self.units.iter().map(|unit| {
            // FIXME: `generate_module` inlined because of borrowing rules: The test runner thinks that
            // self is being borrowed by the internal method
            let mut code_generator =
                plc::codegen::CodeGen::new(&context, root.as_deref(), &unit.file_name, optimization, debug_level);
            //Create a types codegen, this contains all the type declarations
            //Associate the index type with LLVM types
            let llvm_index =
                code_generator.generate_llvm_index(&context, &self.annotations, &self.literals, &self.index)?;
            code_generator.generate(&context, &unit, &self.annotations, &self.index, &llvm_index)
        }).reduce(|a,b| {
            let a = a?;
            let b = b?;
            a.merge(b)
        }) else {
            return Ok(None)
        };
        module.map(|it| Some(it))
    }

    fn generate_module<'ctx>(
        &'ctx self,
        context: &'ctx CodegenContext,
        root: Option<&Path>,
        unit: &CompilationUnit,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Result<GeneratedModule, Diagnostic> {
        let mut code_generator =
            plc::codegen::CodeGen::new(&context, root.as_deref(), &unit.file_name, optimization, debug_level);
        //Create a types codegen, this contains all the type declarations
        //Associate the index type with LLVM types
        let llvm_index =
            code_generator.generate_llvm_index(&context, &self.annotations, &self.literals, &self.index)?;
        code_generator.generate(&context, &unit, &self.annotations, &self.index, &llvm_index)
    }

    pub fn codegen(
        &self,
        root: Option<&Path>,
        build_location: Option<&Path>,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
        format: FormatOption,
        targets: &[Target],
    ) -> Result<GeneratedProject, Diagnostic> {
        let compile_directory = dbg!(build_location).map(|it| it.to_path_buf()).unwrap_or_else(|| {
            let tempdir = tempfile::tempdir().unwrap();
            tempdir.into_path()
        });

        ensure_compile_dirs(targets, &compile_directory)?;
        println!("After create temp");

        let objects = self
            .units
            .par_iter()
            .map(|unit| {
                let unit_location = PathBuf::from(&unit.file_name);
                let output_name = unit_location.file_name().expect("Unit has a filename");
                //For each target compile the module once
                let targets = if targets.is_empty() { &[Target::System] } else { targets };
                targets
                    // TODO: We can't transmit codegen through threads
                    .par_iter()
                    .map(|target| {
                        let context = CodegenContext::new(); //Create a build location for the generated object files
                        let module = self.generate_module(&context, root, unit, optimization, debug_level)?;
                        module
                            .persist(&compile_directory, &output_name.to_string_lossy(), format, target, optimization)
                            .and_then(|it| TryInto::<Object>::try_into(it.as_path()))
                    })
                    .collect::<Result<Vec<Object>, Diagnostic>>()
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(GeneratedProject { objects })
    }
}

/// Ensures the directores for the various targets have been created
fn ensure_compile_dirs(targets: &[Target], compile_directory: &Path) -> Result<(), Diagnostic> {
    for target in targets {
        if let Some(name) = target.try_get_name() {
            let dir = compile_directory.join(name);
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

/// A project that has been transformed into a binary representation
/// Can be linked to generate a usable application
pub struct GeneratedProject {
    objects: Vec<Object>,
}

impl GeneratedProject {
    pub fn link(&self, link_options: LinkOptions) -> Result<Object, Diagnostic> {
        todo!()
    }
}
