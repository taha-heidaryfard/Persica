pub mod arrays;
pub mod math;
pub mod stats;
pub mod strings;

use crate::{
    ast::Expr,
    diagnostic::{Diagnostic, Phase},
    runtime::Value,
};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Int,
    Float,
    Bool,
    String,
    IntArray,
    FloatArray,
    StringArray,
    BoolArray,
    Int2DArray,
    Float2DArray,
    String2DArray,
    AnyArray,
    NumericArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnType {
    Int,
    Float,
    Bool,
    String,
    IntArray,
    FloatArray,
    StringArray,
    BoolArray,
    Int2DArray,
    Float2DArray,
    String2DArray,
    SameAsArg(usize),
    ElementOfArg(usize),
    ArrayOfArg(usize),
}

pub struct Builtin {
    pub name: &'static str,
    pub params: &'static [ParamType],
    pub return_type: ReturnType,
    pub handler: fn(&[Value], &Expr) -> Result<Value, Diagnostic>,
}

static REGISTRY: LazyLock<HashMap<&'static str, Builtin>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for builtin in math::register() {
        map.insert(builtin.name, builtin);
    }
    for builtin in strings::register() {
        map.insert(builtin.name, builtin);
    }
    for builtin in arrays::register() {
        map.insert(builtin.name, builtin);
    }
    for builtin in stats::register() {
        map.insert(builtin.name, builtin);
    }
    map
});

pub fn find_builtin(name: &str) -> Option<&'static Builtin> {
    REGISTRY.get(name)
}

pub fn is_builtin(name: &str) -> bool {
    REGISTRY.contains_key(name)
}

pub fn all_builtin_names() -> Vec<&'static str> {
    let mut names: Vec<_> = REGISTRY.keys().copied().collect();
    names.sort();
    names
}

pub(crate) fn runtime_error(expression: &Expr, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(Phase::Runtime, message, expression.span)
}
