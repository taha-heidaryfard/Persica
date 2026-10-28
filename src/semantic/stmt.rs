use super::{
    Analyzer,
    symbol::{VariableSymbol, error, error_expr},
};
use crate::{
    ast::{Stmt, StmtKind, Type},
    diagnostic::{Diagnostic, Phase},
};

impl Analyzer {
    pub(super) fn statement(&mut self, statement: &Stmt) -> Result<(), Diagnostic> {
        match &statement.kind {
            StmtKind::VariableDeclaration {
                is_final,
                variable_type,
                name,
                initializer,
            } => {
                let actual = self.expression_type_as(initializer, variable_type)?;
                self.require_type(variable_type, &actual, initializer)?;
                let scope = self.scopes.last_mut().expect("analyzer always has a scope");
                if scope.contains_key(name) {
                    return Err(error(
                        statement,
                        format!("variable '{name}' is already defined in this scope"),
                    ));
                }
                scope.insert(
                    name.clone(),
                    VariableSymbol {
                        variable_type: variable_type.clone(),
                        is_final: *is_final,
                    },
                );
            }
            StmtKind::FunctionDeclaration {
                return_type,
                name: _,
                parameters,
                body,
            } => {
                if self.scopes.len() != 1 {
                    return Err(error(
                        statement,
                        "functions may only be declared at the top level",
                    ));
                }
                self.push_scope();
                for parameter in parameters {
                    let scope = self.scopes.last_mut().unwrap();
                    if scope
                        .insert(
                            parameter.name.clone(),
                            VariableSymbol {
                                variable_type: parameter.parameter_type.clone(),
                                is_final: false,
                            },
                        )
                        .is_some()
                    {
                        self.pop_scope();
                        return Err(Diagnostic::new(
                            Phase::Semantic,
                            format!("duplicate parameter '{}'", parameter.name),
                            parameter.span,
                        ));
                    }
                }
                let previous_return = self.current_return.replace(return_type.clone());
                let result = body.iter().try_for_each(|item| self.statement(item));
                self.current_return = previous_return;
                self.pop_scope();
                result?;
                if *return_type != Type::Void && !statements_return(body) {
                    return Err(error(
                        statement,
                        format!("function must return a {return_type} value on every path"),
                    ));
                }
            }
            StmtKind::Import { .. } => {}
            StmtKind::StructDeclaration { .. } => {
                if self.scopes.len() != 1 {
                    return Err(error(
                        statement,
                        "structs may only be declared at the top level",
                    ));
                }
            }
            StmtKind::ClassDeclaration { name, methods, .. } => {
                if self.scopes.len() != 1 {
                    return Err(error(
                        statement,
                        "classes may only be declared at the top level",
                    ));
                }
                for method in methods {
                    self.push_scope();
                    self.scopes.last_mut().unwrap().insert(
                        "this".to_string(),
                        VariableSymbol {
                            variable_type: Type::Custom(name.clone()),
                            is_final: true,
                        },
                    );
                    for parameter in &method.parameters {
                        let scope = self.scopes.last_mut().unwrap();
                        if scope
                            .insert(
                                parameter.name.clone(),
                                VariableSymbol {
                                    variable_type: parameter.parameter_type.clone(),
                                    is_final: false,
                                },
                            )
                            .is_some()
                        {
                            self.pop_scope();
                            return Err(Diagnostic::new(
                                Phase::Semantic,
                                format!("duplicate parameter '{}'", parameter.name),
                                parameter.span,
                            ));
                        }
                    }
                    let previous_class = self.current_class.replace(name.clone());
                    let previous_return = self.current_return.replace(method.return_type.clone());
                    let result = method.body.iter().try_for_each(|item| self.statement(item));
                    self.current_return = previous_return;
                    self.current_class = previous_class;
                    self.pop_scope();
                    result?;
                    if method.return_type != Type::Void && !statements_return(&method.body) {
                        return Err(Diagnostic::new(
                            Phase::Semantic,
                            format!(
                                "method '{}' must return a {} value on every path",
                                method.name, method.return_type
                            ),
                            method.span,
                        ));
                    }
                }
            }
            StmtKind::Block(statements) => {
                self.push_scope();
                let result = statements.iter().try_for_each(|item| self.statement(item));
                self.pop_scope();
                result?;
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expect_expression(condition, &Type::Bool)?;
                self.statement(then_branch)?;
                if let Some(branch) = else_branch {
                    self.statement(branch)?;
                }
            }
            StmtKind::While { condition, body } => {
                self.expect_expression(condition, &Type::Bool)?;
                self.in_loop(|analyzer| analyzer.statement(body))?;
            }
            StmtKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();
                let result = (|| {
                    if let Some(initializer) = initializer {
                        self.statement(initializer)?;
                    }
                    if let Some(condition) = condition {
                        self.expect_expression(condition, &Type::Bool)?;
                    }
                    if let Some(increment) = increment {
                        self.expression_type(increment)?;
                    }
                    self.in_loop(|analyzer| analyzer.statement(body))
                })();
                self.pop_scope();
                result?;
            }
            StmtKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                let element_type = match self.expression_type(iterable)? {
                    Type::Array(element) => *element,
                    Type::String => Type::String,
                    Type::Map { key, .. } => *key,
                    Type::Any => Type::Any,
                    actual => {
                        return Err(error_expr(
                            iterable,
                            format!("for-in expects an array, string, or map, found {actual}"),
                        ));
                    }
                };
                self.push_scope();
                self.scopes.last_mut().unwrap().insert(
                    variable.clone(),
                    VariableSymbol {
                        variable_type: element_type,
                        is_final: false,
                    },
                );
                let result = self.in_loop(|analyzer| analyzer.statement(body));
                self.pop_scope();
                result?;
            }
            StmtKind::Break | StmtKind::Continue => {
                if self.loop_depth == 0 {
                    return Err(error(
                        statement,
                        "loop control can only be used inside a loop",
                    ));
                }
            }
            StmtKind::Return(value) => {
                let Some(expected) = self.current_return.clone() else {
                    return Err(error(
                        statement,
                        "return can only be used inside a function",
                    ));
                };
                match (expected, value) {
                    (Type::Void, None) => {}
                    (Type::Void, Some(_)) => {
                        return Err(error(statement, "a void function cannot return a value"));
                    }
                    (expected, Some(value)) => {
                        let actual = self.expression_type_as(value, &expected)?;
                        self.require_type(&expected, &actual, value)?;
                    }
                    (expected, None) => {
                        return Err(error(
                            statement,
                            format!("expected a {expected} return value"),
                        ));
                    }
                }
            }
            StmtKind::Print {
                arguments,
                separator,
                end,
            } => {
                for argument in arguments {
                    if self.expression_type(argument)? == Type::Void {
                        return Err(error_expr(argument, "cannot print a void value"));
                    }
                }
                if let Some(separator) = separator {
                    self.expect_expression(separator, &Type::String)?;
                }
                if let Some(end) = end {
                    self.expect_expression(end, &Type::String)?;
                }
            }
            StmtKind::Expression(expression) => {
                self.expression_type(expression)?;
            }
        }
        Ok(())
    }
}

pub(super) fn statements_return(statements: &[Stmt]) -> bool {
    statements.iter().any(statement_returns)
}

pub(super) fn statement_returns(statement: &Stmt) -> bool {
    match &statement.kind {
        StmtKind::Return(_) => true,
        StmtKind::Block(items) => statements_return(items),
        StmtKind::If {
            then_branch,
            else_branch: Some(else_branch),
            ..
        } => statement_returns(then_branch) && statement_returns(else_branch),
        _ => false,
    }
}
