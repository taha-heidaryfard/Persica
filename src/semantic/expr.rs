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
            ExprKind::Map(entries) => {
                let Some((first_k, first_v)) = entries.first() else {
                    return Ok(Type::dict());
                };
                let mut key_type = self.expression_type(first_k)?;
                let mut val_type = self.expression_type(first_v)?;
                if key_type == Type::Void {
                    return Err(error_expr(first_k, "void values cannot be map keys"));
                }
                if val_type == Type::Void {
                    return Err(error_expr(first_v, "void values cannot be map values"));
                }
                for (k, v) in &entries[1..] {
                    let k_actual = self.expression_type(k)?;
                    let v_actual = self.expression_type(v)?;
                    if k_actual != key_type {
                        key_type = Type::Any;
                    }
                    if v_actual != val_type {
                        val_type = Type::Any;
                    }
                }
                Ok(Type::map(key_type, val_type))
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
                let target_type = self.expression_type(target)?;
                match target_type {
                    Type::Array(element) => {
                        self.expect_expression(index, &Type::Int)?;
                        let actual = self.expression_type_as(value, &element)?;
                        self.require_type(&element, &actual, value)?;
                        Ok(*element)
                    }
                    Type::Map {
                        key,
                        value: val_type,
                    } => {
                        let actual_key = self.expression_type(index)?;
                        self.require_type(&key, &actual_key, index)?;
                        let actual_val = self.expression_type_as(value, &val_type)?;
                        self.require_type(&val_type, &actual_val, value)?;
                        Ok(*val_type)
                    }
                    Type::Any => {
                        self.expression_type(index)?;
                        self.expression_type(value)?;
                        Ok(Type::Any)
                    }
                    actual => Err(error_expr(
                        target,
                        format!("cannot index-assign a {actual} value"),
                    )),
                }
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
                let target_type = self.expression_type(target)?;
                match target_type {
                    Type::Array(element) => {
                        self.expect_expression(index, &Type::Int)?;
                        Ok(*element)
                    }
                    Type::String => {
                        self.expect_expression(index, &Type::Int)?;
                        Ok(Type::String)
                    }
                    Type::Map { key, value } => {
                        let actual_key = self.expression_type(index)?;
                        self.require_type(&key, &actual_key, index)?;
                        Ok(*value)
                    }
                    Type::Any => {
                        self.expression_type(index)?;
                        Ok(Type::Any)
                    }
                    actual => Err(error_expr(target, format!("cannot index a {actual} value"))),
                }
            }
            ExprKind::This => {
                if let Some(class_name) = &self.current_class {
                    Ok(Type::Custom(class_name.clone()))
                } else {
                    Err(error_expr(
                        expression,
                        "'this' can only be used inside class methods",
                    ))
                }
            }
            ExprKind::Get { object, name } => {
                let target_type = self.expression_type(object)?;
                match target_type {
                    Type::Any => Ok(Type::Any),
                    Type::Custom(type_name) => {
                        if let Some(st) = self.structs.get(&type_name) {
                            if let Some((_, field_type)) = st.fields.iter().find(|(n, _)| n == name)
                            {
                                return Ok(field_type.clone());
                            }
                            return Err(error_expr(
                                expression,
                                format!("struct '{type_name}' has no field '{name}'"),
                            ));
                        }
                        if let Some(cl) = self.classes.get(&type_name) {
                            if let Some((_, field_type)) = cl.fields.iter().find(|(n, _)| n == name)
                            {
                                return Ok(field_type.clone());
                            }
                            return Err(error_expr(
                                expression,
                                format!("class '{type_name}' has no field '{name}'"),
                            ));
                        }
                        Err(error_expr(
                            expression,
                            format!("undefined type '{type_name}'"),
                        ))
                    }
                    actual => Err(error_expr(
                        object,
                        format!("cannot access property '{name}' on type '{actual}'"),
                    )),
                }
            }
            ExprKind::Set {
                object,
                name,
                value,
            } => {
                let target_type = self.expression_type(object)?;
                match target_type {
                    Type::Any => {
                        self.expression_type(value)?;
                        Ok(Type::Any)
                    }
                    Type::Custom(type_name) => {
                        let field_type = if let Some(st) = self.structs.get(&type_name) {
                            st.fields
                                .iter()
                                .find(|(n, _)| n == name)
                                .map(|(_, t)| t.clone())
                        } else if let Some(cl) = self.classes.get(&type_name) {
                            cl.fields
                                .iter()
                                .find(|(n, _)| n == name)
                                .map(|(_, t)| t.clone())
                        } else {
                            return Err(error_expr(
                                expression,
                                format!("undefined type '{type_name}'"),
                            ));
                        };
                        let Some(expected) = field_type else {
                            return Err(error_expr(
                                expression,
                                format!("type '{type_name}' has no field '{name}'"),
                            ));
                        };
                        let actual = self.expression_type_as(value, &expected)?;
                        self.require_type(&expected, &actual, value)?;
                        Ok(expected)
                    }
                    actual => Err(error_expr(
                        object,
                        format!("cannot set property '{name}' on type '{actual}'"),
                    )),
                }
            }
            ExprKind::MethodCall {
                object,
                method,
                arguments,
            } => {
                let target_type = self.expression_type(object)?;
                match target_type {
                    Type::Any => {
                        for arg in arguments {
                            self.expression_type(arg)?;
                        }
                        Ok(Type::Any)
                    }
                    Type::Custom(type_name) => {
                        if self.structs.contains_key(&type_name) {
                            return Err(error_expr(
                                expression,
                                format!("struct '{type_name}' has no methods"),
                            ));
                        }
                        let Some(class_info) = self.classes.get(&type_name).cloned() else {
                            return Err(error_expr(
                                expression,
                                format!("undefined class '{type_name}'"),
                            ));
                        };
                        let Some(signature) = class_info.methods.get(method) else {
                            return Err(error_expr(
                                expression,
                                format!("class '{type_name}' has no method '{method}'"),
                            ));
                        };
                        if signature.parameters.len() != arguments.len() {
                            return Err(error_expr(
                                expression,
                                format!(
                                    "method '{method}' expects {} argument{}, found {}",
                                    signature.parameters.len(),
                                    if signature.parameters.len() == 1 {
                                        ""
                                    } else {
                                        "s"
                                    },
                                    arguments.len()
                                ),
                            ));
                        }
                        for (argument, expected) in arguments.iter().zip(&signature.parameters) {
                            let actual = self.expression_type_as(argument, expected)?;
                            self.require_type(expected, &actual, argument)?;
                        }
                        Ok(signature.return_type.clone())
                    }
                    actual => Err(error_expr(
                        object,
                        format!("cannot call method '{method}' on type '{actual}'"),
                    )),
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
        if (matches!(&expression.kind, ExprKind::Array(elements) if elements.is_empty())
            && matches!(expected, Type::Array(_)))
            || (matches!(&expression.kind, ExprKind::Map(entries) if entries.is_empty())
                && matches!(expected, Type::Map { .. }))
        {
            Ok(expected.clone())
        } else if let ExprKind::Map(entries) = &expression.kind
            && let Type::Map {
                key: exp_k,
                value: exp_v,
            } = expected
        {
            for (k, v) in entries {
                let actual_k = self.expression_type_as(k, exp_k)?;
                self.require_type(exp_k, &actual_k, k)?;
                let actual_v = self.expression_type_as(v, exp_v)?;
                self.require_type(exp_v, &actual_v, v)?;
            }
            Ok(expected.clone())
        } else {
            self.expression_type(expression)
        }
    }

    pub(super) fn is_assignable(expected: &Type, actual: &Type) -> bool {
        if expected == actual || expected == &Type::Any || actual == &Type::Any {
            return true;
        }
        match (expected, actual) {
            (Type::Array(exp_elem), Type::Array(act_elem)) => {
                Self::is_assignable(exp_elem, act_elem)
            }
            (
                Type::Map {
                    key: exp_k,
                    value: exp_v,
                },
                Type::Map {
                    key: act_k,
                    value: act_v,
                },
            ) => Self::is_assignable(exp_k, act_k) && Self::is_assignable(exp_v, act_v),
            _ => false,
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
        if Self::is_assignable(expected, actual) {
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
        if let Some(symbol) = self.scopes.iter().rev().find_map(|scope| scope.get(name)) {
            return Ok(symbol.clone());
        }
        if let Some(class_name) = &self.current_class
            && let Some(class_info) = self.classes.get(class_name)
            && let Some((_, field_type)) = class_info.fields.iter().find(|(n, _)| n == name)
        {
            return Ok(VariableSymbol {
                variable_type: field_type.clone(),
                is_final: false,
            });
        }
        Err(error_expr(
            expression,
            format!("undefined variable '{name}'"),
        ))
    }
}
