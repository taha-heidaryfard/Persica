use crate::{
    ast::{Expr, Stmt, Type},
    diagnostic::{Diagnostic, Phase},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(super) struct FunctionSignature {
    pub(super) parameters: Vec<Type>,
    pub(super) return_type: Type,
}

#[derive(Debug, Clone)]
pub(super) struct VariableSymbol {
    pub(super) variable_type: Type,
    pub(super) is_final: bool,
}

#[derive(Debug, Clone)]
pub(super) struct StructInfo {
    pub(super) fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone)]
pub(super) struct ClassInfo {
    pub(super) fields: Vec<(String, Type)>,
    pub(super) methods: HashMap<String, FunctionSignature>,
}

pub(super) fn require_arity(
    callee: &str,
    arguments: &[Expr],
    expected: usize,
    expression: &Expr,
) -> Result<(), Diagnostic> {
    if arguments.len() != expected {
        Err(error_expr(
            expression,
            format!(
                "{callee} expects {expected} argument{}, found {}",
                if expected == 1 { "" } else { "s" },
                arguments.len()
            ),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn require_pair(
    expected: &Type,
    left: &Type,
    right: &Type,
    expression: &Expr,
) -> Result<(), Diagnostic> {
    if left == expected && right == expected {
        Ok(())
    } else {
        Err(error_expr(
            expression,
            format!("expected {expected} and {expected}, found {left} and {right}"),
        ))
    }
}

pub(super) fn numeric_type(
    actual: Type,
    expression: &Expr,
    name: &str,
) -> Result<Type, Diagnostic> {
    if matches!(actual, Type::Int | Type::Float) {
        Ok(actual)
    } else {
        Err(error_expr(
            expression,
            format!("{name} expects int or float, found {actual}"),
        ))
    }
}

pub(super) fn error(statement: &Stmt, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(Phase::Semantic, message, statement.span)
}

pub(super) fn error_expr(expression: &Expr, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(Phase::Semantic, message, expression.span)
}
