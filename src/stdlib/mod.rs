use crate::ast::Program;
use std::sync::LazyLock;

pub const PRELUDE_SOURCE: &str = include_str!("prelude.persi");

// std/*
pub const STD_COLLECTIONS_SOURCE: &str = include_str!("std/collections.persi");
pub const STD_ALGORITHMS_SOURCE: &str = include_str!("std/algorithms.persi");
pub const STD_STRINGS_SOURCE: &str = include_str!("std/strings.persi");
pub const STD_MATH_SOURCE: &str = include_str!("std/math.persi");
pub const STD_MOD_SOURCE: &str = include_str!("std/mod.persi");

// strings/*
pub const STRINGS_MOD_SOURCE: &str = include_str!("strings/mod.persi");
pub const STRINGS_ALI_SOURCE: &str = include_str!("strings/ali/mod.persi");

// collections/*
pub const COLLECTIONS_MOD_SOURCE: &str = include_str!("collections/mod.persi");

// algorithms/*
pub const ALGORITHMS_MOD_SOURCE: &str = include_str!("algorithms/mod.persi");

// math/*
pub const MATH_MOD_SOURCE: &str = include_str!("math/mod.persi");

pub fn get_stdlib_module(module_specifier: &str) -> Option<&'static str> {
    match module_specifier {
        "std" => Some(STD_MOD_SOURCE),
        "std.collections" => Some(STD_COLLECTIONS_SOURCE),
        "collections" => Some(COLLECTIONS_MOD_SOURCE),
        "std.algorithms" => Some(STD_ALGORITHMS_SOURCE),
        "algorithms" => Some(ALGORITHMS_MOD_SOURCE),
        "std.strings" => Some(STD_STRINGS_SOURCE),
        "strings" => Some(STRINGS_MOD_SOURCE),
        "strings.ali" => Some(STRINGS_ALI_SOURCE),
        "std.math" => Some(STD_MATH_SOURCE),
        "math" => Some(MATH_MOD_SOURCE),
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
