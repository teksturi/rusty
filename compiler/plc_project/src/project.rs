use std::{
    env,
    fs::read_dir,
    path::{Path, PathBuf},
};

use diagnostics::Diagnostic;
use glob::glob;

use crate::{
    build_config::{LinkageInfo, ProjectConfig},
    object::Object,
};

use source_code::{SourceContainer, SourceType};

#[derive(Debug)]
pub enum Linkage {
    Static,
    Shared(Package),
}

/// How a library is intended to be packaged for the project
#[derive(Debug)]
pub enum Package {
    /// The library is available locally, it needs to be shipped with the project
    Local,
    /// The library is available on the target system, no need to ship it
    System,
}

/// Representation of a PLC Library
#[derive(Debug)]
pub enum Library<T: SourceContainer> {
    Compiled(CompiledLibrary<T>),
    Source(Project<T>),
}

/// A Compiled library to be included in the project
#[derive(Debug)]
pub struct CompiledLibrary<T: SourceContainer> {
    name: String,
    //TODO: Version
    /// Location of the header files to be included in the project
    headers: Vec<T>,
    /// Objects files for the compiled library
    objects: Vec<Object>,
    architectures: Vec<String>,
}

/// The information required by a project to successfully include a library
#[derive(Debug)]
pub struct LibraryInformation<T: SourceContainer> {
    /// Location of the library if available
    location: Option<PathBuf>,
    /// Library name, this will be used when including the library
    name: String,
    /// How should the library be linked
    linkage: Linkage,
    /// The actual library in question
    library: Library<T>,
}

/// A PLC project to build
#[derive(Debug)]
pub struct Project<T: SourceContainer> {
    /// Name of the project
    name: String,
    /// The full path for the project, i.e where the build description exists
    location: Option<PathBuf>,
    //TODO: Version
    /// Source code for the project
    sources: Vec<T>,
    /// Files that will be referenced in the project but are not to be compiled (headers)
    includes: Vec<T>,
    /// Object files that do not need to be compiled
    objects: Vec<Object>,
    /// Libraries included in the project configuration
    libraries: Vec<LibraryInformation<T>>,
}

impl <T: SourceContainer> LibraryInformation<T> {
    pub fn get_includes(&self) -> &[T] {
        match &self.library {
            Library::Compiled(lib) => &lib.headers,
            Library::Source(lib) => lib.get_sources(),
        }
    }
}


impl <T: SourceContainer> Project<T> {
    pub fn get_sources(&self) -> &[T] {
        &self.sources
    }
    pub fn get_includes(&self) -> &[T] {
        &self.includes
    }
    
    pub fn get_libraries(&self) -> &[LibraryInformation<T>] {
        &self.libraries
    }
}

//configuration
impl Project<PathBuf> {
    /// Retrieve a project for compilation from a json description
    pub fn from_config(config: &Path) -> Result<Self, Diagnostic> {
        let project_config = ProjectConfig::from_file(config)?;

        let libraries = project_config
            .libraries
            .into_iter()
            .map(|conf| {
                let lib_path = config.parent().map(|it| it.join(&conf.path))
                    .unwrap_or_else(|| conf.path);
                //TODO: Find all lib objects
                let mut objects = vec![];
                for arch in &conf.architectures {
                    let path = lib_path.join(arch);
                    for item in read_dir(path)? {
                        let item = item?;
                        let object = item.path().as_path().try_into()?;
                        objects.push(object);
                    }
                }

                let compiled_library = CompiledLibrary {
                    name: conf.name.clone(),
                    objects,
                    headers: resolve_file_paths(conf.include_path)?,
                    architectures: conf.architectures,
                };
                Ok(LibraryInformation {
                    name: conf.name,
                    location: Some(lib_path),
                    linkage: conf.package.into(),
                    library: Library::Compiled(compiled_library),
                })
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        let current_dir = env::current_dir()?;
        let location = config.parent().map(|it| it.to_path_buf()).or_else(|| Some(current_dir));
        
        Ok(Project {
            name: project_config.name,
            location,
            sources: resolve_file_paths(project_config.files)?,
            includes: vec![],
            objects: vec![],
            libraries,
        })
    }


    pub fn with_file_pathes(self, files: Vec<PathBuf>) -> Self {
        let mut proj = self;
        let files = resolve_file_paths(files).unwrap();
        for file in files {
            if matches!(file.get_type(), SourceType::Unknown) {
                let obj = file.as_path().try_into().unwrap();
                proj.objects.push(obj);
            } else {
                proj.sources.push(file);
            }
        }
        proj
    }

    pub fn with_include_pathes(self, files: Vec<PathBuf>) -> Self {
        let mut proj = self;
        proj.includes = resolve_file_paths(files).unwrap();
        proj
    }

}

impl <S: SourceContainer> Project<S> {

    pub fn new(name: String) -> Self {
        Project { name, location: None, sources: vec![], includes: vec![], objects: vec![], libraries: vec![] }
    }

    pub fn with_sources<T: IntoIterator<Item = S>>(mut self, sources: T) -> Self {
        self.sources.extend(sources);
        self
    }

    pub fn with_source_includes<T: IntoIterator<Item = S>>(mut self, includes: T) -> Self {
        self.includes.extend(includes);
        self
    }

    pub fn with_libraries(self, libraries: Vec<String>) -> Self {
        let mut proj = self;
        for library in libraries {
            proj.libraries.push(LibraryInformation {
                name: library.to_string(),
                location: None,
                linkage: Linkage::Shared(Package::System),
                library: Library::Compiled(CompiledLibrary {
                    name: library.to_string(),
                    headers: vec![],
                    objects: vec![],
                    architectures: vec![],
                }),
            });
        }
        proj
    }

    pub fn get_location(&self) -> Option<&Path> {
       self.location.as_deref()
    }

}

fn resolve_file_paths(inputs: Vec<PathBuf>) -> Result<Vec<PathBuf>, Diagnostic> {
    let mut sources = Vec::new();
    for input in inputs {
        let path = &input.to_string_lossy();
        let paths = glob(path)
            .map_err(|e| Diagnostic::param_error(&format!("Failed to read glob pattern: {path}, ({e})")))?;

        for p in paths {
            let path = p.map_err(|err| Diagnostic::param_error(&format!("Illegal path: {err}")))?;
            sources.push(path);
        }
    }
    Ok(sources)
}

impl From<LinkageInfo> for Linkage {
    fn from(value: LinkageInfo) -> Self {
        match value {
            LinkageInfo::Copy | LinkageInfo::Local => Self::Shared(Package::Local),
            LinkageInfo::System => Self::Shared(Package::System),
            LinkageInfo::Static => Self::Static,
        }
    }
}
