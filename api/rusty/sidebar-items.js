initSidebarItems({"enum":[["ConfigFormat",""],["ErrorFormat",""],["FormatOption",""],["OptimizationLevel",""]],"fn":[["build","The builder function for the compilation Sorts files that need compilation Parses, validates and generates code for the given source files Persists the generated code to output location Returns a compilation result with the index, and a list of object files"],["build_with_params","The driver function for the compilation Sorts files that need compilation Parses, validates and generates code for the given source files Links all provided object files with the compilation result Links any provided libraries Returns the location of the output file"],["compile_module","Compiles the given source into a `codegen::CodeGen` using the provided context"],["get_target_triple",""],["link",""],["persist",""],["persist_as_static_obj","Persists a given LLVM module to a static object and saves the output."],["persist_to_bitcode","Persists the given LLVM module into a bitcode file"],["persist_to_ir","Persits the given LLVM module into LLVM IR and saves it to the given output location"],["persist_to_shared_object","Persists the given LLVM module to a dynamic non PIC object and saves the output."],["persist_to_shared_pic_object","Persists a given LLVM module to a shared postiion indepedent object and saves the output."]],"macro":[["expect_token",""]],"mod":[["cli",""],["diagnostics",""],["expression_path",""],["index",""],["runner",""]],"struct":[["CompileOptions",""],["CompileResult","A struct representing the result of a compilation"],["FilePath",""],["LinkOptions",""],["SourceCode","The SourceCode unit is the smallest unit of compilation that can be passed to the compiler"]],"trait":[["SourceContainer","SourceContainers offer source-code to be compiled via the load_source function. Furthermore it offers a location-String used when reporting diagnostics."]]});