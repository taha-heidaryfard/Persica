use super::{
    Analyzer,
    symbol::{VariableSymbol, error_expr, require_pair},
};
use crate::{
    ast::{BinaryOp, Expr, ExprKind, Literal, Type, UnaryOp},
    diagnostic::Diagnostic,
};

impl Analyzer {
    pub(super) fn expression_type(&mut self, expression: &Expr) -> Result<Type, Diagnostic> {
        match &expression.kind {
            ExprKind::Literal(Literal::Int(_)) => Ok(Type::Int),
            ExprKind::Literal(Literal::Float(_)) => Ok(Type::Float),
            ExprKind::Literal(Literal::Bool(_)) => Ok(Type::Bool),
            ExprKind::Literal(Literal::String(_)) => Ok(Type::String),
            ExprKind::Array(elements) => {
                let Some(first) = elements.first() else {
                    return Err(error_expr(
                        expression,
                        "cannot infer the type of an empty array",
                    ));
                };
                let element_type = self.expression_type(first)?;
                if element_type == Type::Void {
                    return Err(error_expr(
                        first,
                        "void values cannot be stored in an array",
                    ));
                }
                for element in &elements[1..] {
                    let actual = self.expression_type(element)?;
                    self.require_type(&element_type, &actual, element)?;
                }
                Ok(Type::array(element_type))
            }
            ExprKind::Identifier(name) => self.variable(name, expression),
            ExprKind::Grouping(inner) => self.expression_type(inner),
            ExprKind::Assignment { name, value } => {
                let symbol = self.variable_symbol(name, expression)?;
                if symbol.is_final {
                    return Err(error_expr(
                        expression,
                        format!("cannot assign to final variable '{name}'"),
                    ));
                }
                let expected = symbol.variable_type;
                let actual = self.expression_type_as(value, &expected)?;
                self.require_type(&expected, &actual, value)?;
                Ok(expected)
            }
            ExprKind::IndexAssignment {
                target,
                index,
                value,
            } => {
                self.expect_expression(index, &Type::Int)?;
                let Type::Array(element) = self.expression_type(target)? else {
                    return Err(error_expr(target, "index assignment requires an array"));
                };
                let actual = self.expression_type(value)?;
                self.require_type(&element, &actual, value)?;
                Ok(*element)
            }
            ExprKind::Unary { op, right } => match op {
                UnaryOp::Not => {
                    self.expect_expression(right, &Type::Bool)?;
                    Ok(Type::Bool)
                }
                UnaryOp::Negate => match self.expression_type(right)? {
                    Type::Int => Ok(Type::Int),
                    Type::Float => Ok(Type::Float),
                    actual => Err(error_expr(
                        right,
                        format!("unary '-' expects int or float, found {actual}"),
                    )),
                },
            },
            ExprKind::Binary { left, op, right } => self.binary_type(left, *op, right, expression),
            ExprKind::Call { callee, arguments } => self.call_type(callee, arguments, expression),
            ExprKind::Index { target, index } => {
                self.expect_expression(index, &Type::Int)?;
                match self.expression_type(target)? {
                    Type::Array(element) => Ok(*element),
                    Type::String => Ok(Type::String),
                    actual => Err(error_expr(target, format!("cannot index a {actual} value"))),
                }
            }
        }
    }

    fn binary_type(
        &mut self,
        left: &Expr,
        op: BinaryOp,
        right: &Expr,
        whole: &Expr,
    ) -> Result<Type, Diagnostic> {
        let left_type = self.expression_type(left)?;
        let right_type = self.expression_type(right)?;
        match op {
            BinaryOp::Add if left_type == Type::String && right_type == Type::String => {
                Ok(Type::String)
            }
            BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Modulo => {
                if left_type == Type::Int && right_type == Type::Int {
                    Ok(Type::Int)
                } else if left_type == Type::Float && right_type == Type::Float {
                    Ok(Type::Float)
                } else {
                    Err(error_expr(
                        whole,
                        format!(
                            "numeric operator expects matching int or float operands, found {left_type} and {right_type}"
                        ),
                    ))
                }
            }
            BinaryOp::Greater | BinaryOp::GreaterEqual | BinaryOp::Less | BinaryOp::LessEqual => {
                if (left_type == Type::Int && right_type == Type::Int)
                    || (left_type == Type::Float && right_type == Type::Float)
                {
                    Ok(Type::Bool)
                } else {
                    Err(error_expr(
                        whole,
                        format!(
                            "comparison expects matching int or float operands, found {left_type} and {right_type}"
                        ),
                    ))
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                require_pair(&Type::Bool, &left_type, &right_type, whole)?;
                Ok(Type::Bool)
            }
            BinaryOp::Equal | BinaryOp::NotEqual => {
                if left_type != right_type {
                    return Err(error_expr(
                        whole,
                        format!("cannot compare {left_type} with {right_type}"),
                    ));
                }
                if left_type == Type::Void {
                    return Err(error_expr(whole, "void values cannot be compared"));
                }
                Ok(Type::Bool)
            }
        }
    }

    pub(super) fn expression_type_as(
        &mut self,
        expression: &Expr,
        expected: &Type,
    ) -> Result<Type, Diagnostic> {
        if matches!(&expression.kind, ExprKind::Array(elements) if elements.is_empty())
            && matches!(expected, Type::Array(_))
        {
            Ok(expected.clone())
        } else {
            self.expression_type(expression)
        }
    }

    pub(super) fn expect_expression(
        &mut self,
        expression: &Expr,
        expected: &Type,
    ) -> Result<(), Diagnostic> {
        let actual = self.expression_type(expression)?;
        self.require_type(expected, &actual, expression)
    }

    pub(super) fn require_type(
        &self,
        expected: &Type,
        actual: &Type,
        expression: &Expr,
    ) -> Result<(), Diagnostic> {
        if expected == actual {
            Ok(())
        } else {
            Err(error_expr(
                expression,
                format!("expected {expected}, found {actual}"),
            ))
        }
    }

    fn variable(&self, name: &str, expression: &Expr) -> Result<Type, Diagnostic> {
        Ok(self.variable_symbol(name, expression)?.variable_type)
    }

    fn variable_symbol(&self, name: &str, expression: &Expr) -> Result<VariableSymbol, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .cloned()
            .ok_or_else(|| error_expr(expression, format!("undefined variable '{name}'")))
    }
}
