use super::{Binding, Flow, Interpreter, Value, runtime_expr};
use crate::{
    ast::{BinaryOp, Expr, ExprKind, Literal, Stmt, StmtKind, Type, UnaryOp},
    diagnostic::Diagnostic,
};
use std::collections::HashMap;

impl Interpreter {
    pub(super) fn execute(&mut self, statement: &Stmt) -> Result<Flow, Diagnostic> {
        match &statement.kind {
            StmtKind::VariableDeclaration {
                is_final,
                variable_type,
                name,
                initializer,
            } => {
                let value = self.evaluate(initializer)?;
                self.scopes.last_mut().unwrap().insert(
                    name.clone(),
                    Binding {
                        variable_type: variable_type.clone(),
                        is_final: *is_final,
                        value,
                    },
                );
                Ok(Flow::Normal)
            }
            StmtKind::FunctionDeclaration { .. } => Ok(Flow::Normal),
            StmtKind::Block(statements) => self.execute_scope(statements),
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.boolean(condition)? {
                    self.execute(then_branch)
                } else if let Some(branch) = else_branch {
                    self.execute(branch)
                } else {
                    Ok(Flow::Normal)
                }
            }
            StmtKind::While { condition, body } => {
                while self.boolean(condition)? {
                    match self.execute(body)? {
                        Flow::Normal | Flow::Continue => {}
                        Flow::Break => break,
                        returned @ Flow::Return(_) => return Ok(returned),
                    }
                }
                Ok(Flow::Normal)
            }
            StmtKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.scopes.push(HashMap::new());
                let result = self.execute_for(
                    initializer.as_deref(),
                    condition.as_ref(),
                    increment.as_ref(),
                    body,
                );
                self.scopes.pop();
                result
            }
            StmtKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                let values = match self.evaluate(iterable)? {
                    Value::Array(values) => values,
                    Value::String(value) => value
                        .chars()
                        .map(|character| Value::String(character.to_string()))
                        .collect(),
                    Value::Map(entries) => entries.into_iter().map(|(k, _)| k).collect(),
                    _ => return Err(runtime_expr(iterable, "for-in value is not iterable")),
                };
                self.scopes.push(HashMap::new());
                let mut result = Ok(Flow::Normal);
                for value in values {
                    self.scopes.last_mut().unwrap().insert(
                        variable.clone(),
                        Binding {
                            variable_type: value.value_type(),
                            is_final: false,
                            value,
                        },
                    );
                    match self.execute(body) {
                        Ok(Flow::Normal | Flow::Continue) => {}
                        Ok(Flow::Break) => break,
                        Ok(returned @ Flow::Return(_)) => {
                            result = Ok(returned);
                            break;
                        }
                        Err(error) => {
                            result = Err(error);
                            break;
                        }
                    }
                }
                self.scopes.pop();
                result
            }
            StmtKind::Break => Ok(Flow::Break),
            StmtKind::Continue => Ok(Flow::Continue),
            StmtKind::Return(value) => Ok(Flow::Return(match value {
                Some(expression) => self.evaluate(expression)?,
                None => Value::Void,
            })),
            StmtKind::Print {
                arguments,
                separator,
                end,
            } => {
                let separator = match separator {
                    Some(expression) => self.evaluate_string(expression)?,
                    None => " ".to_owned(),
                };
                let end = match end {
                    Some(expression) => self.evaluate_string(expression)?,
                    None => "\n".to_owned(),
                };
                let values = arguments
                    .iter()
                    .map(|argument| self.evaluate(argument).map(|value| value.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;
                let content = values.join(&separator);
                self.print_calls.push(content.clone());
                self.emit(&(content + &end))?;
                Ok(Flow::Normal)
            }
            StmtKind::Expression(expression) => {
                self.evaluate(expression)?;
                Ok(Flow::Normal)
            }
        }
    }

    pub(super) fn execute_scope(&mut self, statements: &[Stmt]) -> Result<Flow, Diagnostic> {
        self.scopes.push(HashMap::new());
        let result = self.execute_statements(statements);
        self.scopes.pop();
        result
    }

    pub(super) fn execute_statements(&mut self, statements: &[Stmt]) -> Result<Flow, Diagnostic> {
        for statement in statements {
            let flow = self.execute(statement)?;
            if !matches!(flow, Flow::Normal) {
                return Ok(flow);
            }
        }
        Ok(Flow::Normal)
    }

    fn execute_for(
        &mut self,
        initializer: Option<&Stmt>,
        condition: Option<&Expr>,
        increment: Option<&Expr>,
        body: &Stmt,
    ) -> Result<Flow, Diagnostic> {
        if let Some(initializer) = initializer {
            self.execute(initializer)?;
        }
        loop {
            if let Some(condition) = condition
                && !self.boolean(condition)?
            {
                break;
            }
            match self.execute(body)? {
                Flow::Break => break,
                returned @ Flow::Return(_) => return Ok(returned),
                Flow::Normal | Flow::Continue => {}
            }
            if let Some(increment) = increment {
                self.evaluate(increment)?;
            }
        }
        Ok(Flow::Normal)
    }

    fn boolean(&mut self, expression: &Expr) -> Result<bool, Diagnostic> {
        match self.evaluate(expression)? {
            Value::Bool(value) => Ok(value),
            _ => Err(runtime_expr(
                expression,
                "condition did not evaluate to bool",
            )),
        }
    }

    pub(super) fn evaluate(&mut self, expression: &Expr) -> Result<Value, Diagnostic> {
        match &expression.kind {
            ExprKind::Literal(Literal::Int(value)) => Ok(Value::Int(*value)),
            ExprKind::Literal(Literal::Float(value)) => Ok(Value::Float(*value)),
            ExprKind::Literal(Literal::Bool(value)) => Ok(Value::Bool(*value)),
            ExprKind::Literal(Literal::String(value)) => Ok(Value::String(value.clone())),
            ExprKind::Array(elements) => elements
                .iter()
                .map(|element| self.evaluate(element))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array),
            ExprKind::Map(entries) => {
                let mut map_entries = Vec::new();
                for (key_expr, val_expr) in entries {
                    let key = self.evaluate(key_expr)?;
                    let val = self.evaluate(val_expr)?;
                    if let Some(existing) = map_entries.iter_mut().find(|(k, _)| k == &key) {
                        existing.1 = val;
                    } else {
                        map_entries.push((key, val));
                    }
                }
                Ok(Value::Map(map_entries))
            }
            ExprKind::Identifier(name) => self
                .get(name)
                .map(|binding| binding.value.clone())
                .ok_or_else(|| runtime_expr(expression, format!("undefined variable '{name}'"))),
            ExprKind::Grouping(inner) => self.evaluate(inner),
            ExprKind::Assignment { name, value } => {
                let value = self.evaluate(value)?;
                let binding = self.get_mut(name).ok_or_else(|| {
                    runtime_expr(expression, format!("undefined variable '{name}'"))
                })?;
                if binding.is_final {
                    return Err(runtime_expr(
                        expression,
                        format!("cannot assign to final variable '{name}'"),
                    ));
                }
                if !value.is_assignable_to(&binding.variable_type) {
                    return Err(runtime_expr(
                        expression,
                        format!(
                            "cannot assign {} to {}",
                            value.value_type(),
                            binding.variable_type
                        ),
                    ));
                }
                binding.value = value.clone();
                Ok(value)
            }
            ExprKind::IndexAssignment {
                target,
                index,
                value,
            } => self.assign_index(target, index, value, expression),
            ExprKind::Unary { op, right } => {
                let value = self.evaluate(right)?;
                match (op, value) {
                    (UnaryOp::Not, Value::Bool(value)) => Ok(Value::Bool(!value)),
                    (UnaryOp::Negate, Value::Int(value)) => {
                        value.checked_neg().map(Value::Int).ok_or_else(|| {
                            runtime_expr(expression, "integer overflow during negation")
                        })
                    }
                    (UnaryOp::Negate, Value::Float(value)) => Ok(Value::Float(-value)),
                    _ => Err(runtime_expr(expression, "invalid unary operand")),
                }
            }
            ExprKind::Binary { left, op, right } => {
                self.evaluate_binary(left, *op, right, expression)
            }
            ExprKind::Call { callee, arguments } => self.call(callee, arguments, expression),
            ExprKind::Index { target, index } => {
                let target = self.evaluate(target)?;
                match target {
                    Value::Array(values) => {
                        let index_num = self.index(index)?;
                        values.get(index_num).cloned().ok_or_else(|| {
                            runtime_expr(
                                expression,
                                format!(
                                    "array index {index_num} is out of bounds for length {}",
                                    values.len()
                                ),
                            )
                        })
                    }
                    Value::String(value) => {
                        let index_num = self.index(index)?;
                        value
                            .chars()
                            .nth(index_num)
                            .map(|ch| Value::String(ch.to_string()))
                            .ok_or_else(|| {
                                runtime_expr(
                                    expression,
                                    format!("string index {index_num} is out of bounds"),
                                )
                            })
                    }
                    Value::Map(entries) => {
                        let key = self.evaluate(index)?;
                        for (k, v) in &entries {
                            if k == &key {
                                return Ok(v.clone());
                            }
                        }
                        Err(runtime_expr(
                            expression,
                            format!("key {key} not found in map"),
                        ))
                    }
                    _ => Err(runtime_expr(expression, "value is not indexable")),
                }
            }
        }
    }

    fn assign_index(
        &mut self,
        target: &Expr,
        index: &Expr,
        value: &Expr,
        whole: &Expr,
    ) -> Result<Value, Diagnostic> {
        let new_value = self.evaluate(value)?;
        let final_key = self.evaluate(index)?;
        let mut indices = Vec::new();
        let name = collect_target(target, &mut indices)
            .ok_or_else(|| runtime_expr(target, "index assignment must start from a variable"))?
            .to_owned();
        let mut evaluated_keys = Vec::new();
        for item in indices {
            evaluated_keys.push(self.evaluate(item)?);
        }
        let binding = self
            .get_mut(&name)
            .ok_or_else(|| runtime_expr(target, format!("undefined variable '{name}'")))?;
        let mut current = &mut binding.value;
        for item in evaluated_keys {
            match current {
                Value::Array(values) => {
                    let idx = Self::to_usize(&item, whole)?;
                    let length = values.len();
                    current = values.get_mut(idx).ok_or_else(|| {
                        runtime_expr(
                            whole,
                            format!("array index {idx} is out of bounds for length {length}"),
                        )
                    })?;
                }
                Value::Map(entries) => {
                    let mut found = None;
                    for (i, (k, _)) in entries.iter().enumerate() {
                        if k == &item {
                            found = Some(i);
                            break;
                        }
                    }
                    let idx = found.ok_or_else(|| {
                        runtime_expr(whole, format!("key {item} not found in map"))
                    })?;
                    current = &mut entries[idx].1;
                }
                _ => {
                    return Err(runtime_expr(
                        whole,
                        "nested index target is not an array or map",
                    ));
                }
            }
        }
        match current {
            Value::Array(values) => {
                let idx = Self::to_usize(&final_key, whole)?;
                let length = values.len();
                let slot = values.get_mut(idx).ok_or_else(|| {
                    runtime_expr(
                        whole,
                        format!("array index {idx} is out of bounds for length {length}"),
                    )
                })?;
                *slot = new_value.clone();
            }
            Value::Map(entries) => {
                if let Some(existing) = entries.iter_mut().find(|(k, _)| k == &final_key) {
                    existing.1 = new_value.clone();
                } else {
                    entries.push((final_key, new_value.clone()));
                }
            }
            _ => {
                return Err(runtime_expr(
                    whole,
                    "index assignment requires an array or map",
                ));
            }
        }
        Ok(new_value)
    }

    fn to_usize(val: &Value, expression: &Expr) -> Result<usize, Diagnostic> {
        match val {
            Value::Int(value) if *value >= 0 => {
                usize::try_from(*value).map_err(|_| runtime_expr(expression, "index is too large"))
            }
            Value::Int(_) => Err(runtime_expr(expression, "index cannot be negative")),
            _ => Err(runtime_expr(expression, "array index must be an int")),
        }
    }

    fn index(&mut self, expression: &Expr) -> Result<usize, Diagnostic> {
        match self.evaluate(expression)? {
            Value::Int(value) if value >= 0 => {
                usize::try_from(value).map_err(|_| runtime_expr(expression, "index is too large"))
            }
            Value::Int(_) => Err(runtime_expr(expression, "index cannot be negative")),
            _ => Err(runtime_expr(expression, "index must be an int")),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        op: BinaryOp,
        right: &Expr,
        whole: &Expr,
    ) -> Result<Value, Diagnostic> {
        let left_value = self.evaluate(left)?;
        if op == BinaryOp::And && left_value == Value::Bool(false) {
            return Ok(Value::Bool(false));
        }
        if op == BinaryOp::Or && left_value == Value::Bool(true) {
            return Ok(Value::Bool(true));
        }
        let right_value = self.evaluate(right)?;
        match (left_value, op, right_value) {
            (Value::Int(left), BinaryOp::Add, Value::Int(right)) => {
                checked(left.checked_add(right), whole, "addition")
            }
            (Value::Int(left), BinaryOp::Subtract, Value::Int(right)) => {
                checked(left.checked_sub(right), whole, "subtraction")
            }
            (Value::Int(left), BinaryOp::Multiply, Value::Int(right)) => {
                checked(left.checked_mul(right), whole, "multiplication")
            }
            (Value::Int(_), BinaryOp::Divide | BinaryOp::Modulo, Value::Int(0)) => {
                Err(runtime_expr(whole, "division by zero"))
            }
            (Value::Int(left), BinaryOp::Divide, Value::Int(right)) => {
                checked(left.checked_div(right), whole, "division")
            }
            (Value::Int(left), BinaryOp::Modulo, Value::Int(right)) => {
                checked(left.checked_rem(right), whole, "remainder")
            }
            (Value::Float(_), BinaryOp::Divide | BinaryOp::Modulo, Value::Float(0.0)) => {
                Err(runtime_expr(whole, "division by zero"))
            }
            (Value::Float(left), BinaryOp::Add, Value::Float(right)) => {
                Ok(Value::Float(left + right))
            }
            (Value::Float(left), BinaryOp::Subtract, Value::Float(right)) => {
                Ok(Value::Float(left - right))
            }
            (Value::Float(left), BinaryOp::Multiply, Value::Float(right)) => {
                Ok(Value::Float(left * right))
            }
            (Value::Float(left), BinaryOp::Divide, Value::Float(right)) => {
                Ok(Value::Float(left / right))
            }
            (Value::Float(left), BinaryOp::Modulo, Value::Float(right)) => {
                Ok(Value::Float(left % right))
            }
            (Value::String(mut left), BinaryOp::Add, Value::String(right)) => {
                left.push_str(&right);
                Ok(Value::String(left))
            }
            (Value::Int(left), BinaryOp::Greater, Value::Int(right)) => {
                Ok(Value::Bool(left > right))
            }
            (Value::Int(left), BinaryOp::GreaterEqual, Value::Int(right)) => {
                Ok(Value::Bool(left >= right))
            }
            (Value::Int(left), BinaryOp::Less, Value::Int(right)) => Ok(Value::Bool(left < right)),
            (Value::Int(left), BinaryOp::LessEqual, Value::Int(right)) => {
                Ok(Value::Bool(left <= right))
            }
            (Value::Float(left), BinaryOp::Greater, Value::Float(right)) => {
                Ok(Value::Bool(left > right))
            }
            (Value::Float(left), BinaryOp::GreaterEqual, Value::Float(right)) => {
                Ok(Value::Bool(left >= right))
            }
            (Value::Float(left), BinaryOp::Less, Value::Float(right)) => {
                Ok(Value::Bool(left < right))
            }
            (Value::Float(left), BinaryOp::LessEqual, Value::Float(right)) => {
                Ok(Value::Bool(left <= right))
            }
            (left, BinaryOp::Equal, right) => Ok(Value::Bool(left == right)),
            (left, BinaryOp::NotEqual, right) => Ok(Value::Bool(left != right)),
            (Value::Bool(left), BinaryOp::And, Value::Bool(right)) => {
                Ok(Value::Bool(left && right))
            }
            (Value::Bool(left), BinaryOp::Or, Value::Bool(right)) => Ok(Value::Bool(left || right)),
            _ => Err(runtime_expr(whole, "invalid operands for binary operator")),
        }
    }

    fn call(
        &mut self,
        callee: &str,
        arguments: &[Expr],
        expression: &Expr,
    ) -> Result<Value, Diagnostic> {
        self.evaluate_call(callee, arguments, expression)
    }

    pub(super) fn call_user_function(
        &mut self,
        name: &str,
        values: Vec<Value>,
        expression: &Expr,
    ) -> Result<Value, Diagnostic> {
        if self.call_depth >= 256 {
            return Err(runtime_expr(expression, "maximum call depth exceeded"));
        }
        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| runtime_expr(expression, format!("undefined function '{name}'")))?;
        self.call_depth += 1;
        self.scopes.push(HashMap::new());
        for (parameter, value) in function.parameters.iter().zip(values) {
            self.scopes.last_mut().unwrap().insert(
                parameter.name.clone(),
                Binding {
                    variable_type: parameter.parameter_type.clone(),
                    is_final: false,
                    value,
                },
            );
        }
        let result = self.execute_statements(&function.body);
        self.scopes.pop();
        self.call_depth -= 1;
        match result? {
            Flow::Return(value) => Ok(value),
            Flow::Normal if function.return_type == Type::Void => Ok(Value::Void),
            Flow::Normal => Err(runtime_expr(
                expression,
                format!("function '{name}' finished without returning"),
            )),
            Flow::Break | Flow::Continue => {
                Err(runtime_expr(expression, "loop control escaped a function"))
            }
        }
    }

    fn evaluate_string(&mut self, expression: &Expr) -> Result<String, Diagnostic> {
        match self.evaluate(expression)? {
            Value::String(value) => Ok(value),
            _ => Err(runtime_expr(expression, "expected a string value")),
        }
    }
}

fn collect_target<'a>(expression: &'a Expr, indices: &mut Vec<&'a Expr>) -> Option<&'a str> {
    match &expression.kind {
        ExprKind::Identifier(name) => Some(name),
        ExprKind::Index { target, index } => {
            let name = collect_target(target, indices)?;
            indices.push(index);
            Some(name)
        }
        _ => None,
    }
}

fn checked(value: Option<i64>, expression: &Expr, operation: &str) -> Result<Value, Diagnostic> {
    value
        .map(Value::Int)
        .ok_or_else(|| runtime_expr(expression, format!("integer overflow during {operation}")))
}
