pub mod ast;
pub mod builtins;
pub mod diagnostic;
pub mod lexer;
pub mod module;
pub mod parser;
pub mod project;
pub mod runtime;
pub mod semantic;
pub mod source;
pub mod stdlib;

pub use module::{
    clear_project_search_paths, clear_virtual_modules, register_virtual_module,
    set_project_search_paths,
};
pub use project::ProjectConfig;

use ast::Program;
use diagnostic::Diagnostic;
use lexer::Token;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Compilation {
    pub tokens: Vec<Token>,
    pub program: Program,
}

pub fn compile(source: &str) -> Result<Compilation, Diagnostic> {
    compile_with_base_dir(source, None)
}

pub fn compile_with_base_dir(
    source: &str,
    base_dir: Option<&Path>,
) -> Result<Compilation, Diagnostic> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::parse(&tokens)?;
    semantic::validate_with_base_dir(&program, base_dir)?;
    Ok(Compilation { tokens, program })
}

pub fn run(source: &str) -> Result<Vec<String>, Diagnostic> {
    run_with_base_dir(source, None)
}

pub fn run_with_base_dir(source: &str, base_dir: Option<&Path>) -> Result<Vec<String>, Diagnostic> {
    let compilation = compile_with_base_dir(source, base_dir)?;
    let mut interpreter = runtime::Interpreter::new();
    if let Some(dir) = base_dir {
        interpreter.set_current_file_dir(dir.to_path_buf());
    }
    interpreter.interpret(&compilation.program)
}
