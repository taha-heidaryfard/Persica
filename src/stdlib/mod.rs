use crate::ast::Program;
use std::sync::LazyLock;

pub const PRELUDE_SOURCE: &str = include_str!("prelude.persi");
pub const COLLECTIONS_SOURCE: &str = include_str!("collections.persi");
pub const ALGORITHMS_SOURCE: &str = include_str!("algorithms.persi");
pub const STRINGS_SOURCE: &str = include_str!("strings.persi");
pub const MATH_SOURCE: &str = include_str!("math.persi");

pub fn get_stdlib_module(module_specifier: &str) -> Option<&'static str> {
    match module_specifier {
        "std.collections" | "collections" => Some(COLLECTIONS_SOURCE),
        "std.algorithms" | "algorithms" => Some(ALGORITHMS_SOURCE),
        "std.strings" | "strings" => Some(STRINGS_SOURCE),
        "std.math" => Some(MATH_SOURCE),
        "std.prelude" => Some(PRELUDE_SOURCE),
        _ => None,
    }
}

pub static PRELUDE_PROGRAM: LazyLock<Program> = LazyLock::new(|| {
    let tokens = crate::lexer::tokenize(PRELUDE_SOURCE).expect("prelude lexing must succeed");
    crate::parser::parse(&tokens).expect("prelude parsing must succeed")
});

pub fn get_prelude_program() -> &'static Program {
    &PRELUDE_PROGRAM
}
