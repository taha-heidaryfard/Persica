pub mod ast;
pub mod builtins;
pub mod diagnostic;
pub mod lexer;
pub mod module;
pub mod parser;
pub mod runtime;
pub mod semantic;
pub mod source;

pub use module::{clear_virtual_modules, register_virtual_module};

use ast::Program;
use diagnostic::Diagnostic;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Compilation {
    pub tokens: Vec<Token>,
    pub program: Program,
}

pub fn compile(source: &str) -> Result<Compilation, Diagnostic> {
    let tokens = lexer::tokenize(source)?;
    let program = parser::parse(&tokens)?;
    semantic::validate(&program)?;
    Ok(Compilation { tokens, program })
}

pub fn run(source: &str) -> Result<Vec<String>, Diagnostic> {
    let compilation = compile(source)?;
    runtime::Interpreter::new().interpret(&compilation.program)
}
