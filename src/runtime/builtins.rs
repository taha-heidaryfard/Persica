use super::{Input, Interpreter, Value, runtime_expr};
use crate::{ast::Expr, diagnostic::Diagnostic};
use std::io::{self, BufRead};

impl Interpreter {
    pub(super) fn evaluate_call(
        &mut self,
        callee: &str,
        arguments: &[Expr],
        expression: &Expr,
    ) -> Result<Value, Diagnostic> {
        let values = arguments
            .iter()
            .map(|argument| self.evaluate(argument))
            .collect::<Result<Vec<_>, _>>()?;
        if self.functions.contains_key(callee) {
            return self.call_user_function(callee, values, expression);
        }
        if let Some(builtin) = crate::builtins::find_builtin(callee) {
            return (builtin.handler)(&values, expression);
        }
        match callee {
            "read_line" | "input" => self.read_line(values.first(), expression),
            "len" => match &values[0] {
                Value::String(value) => Ok(Value::Int(value.chars().count() as i64)),
                Value::Array(value) => Ok(Value::Int(value.len() as i64)),
                _ => Err(runtime_expr(expression, "invalid len argument")),
            },
            "to_string" => Ok(Value::String(values[0].to_string())),
            "to_int" => match &values[0] {
                Value::Int(value) => Ok(Value::Int(*value)),
                Value::Float(value) if value.is_finite() && value.fract() == 0.0 => {
                    float_to_int(*value, expression)
                }
                Value::String(value) => value
                    .parse::<i64>()
                    .map(Value::Int)
                    .map_err(|_| runtime_expr(expression, format!("'{value}' is not a valid int"))),
                _ => Err(runtime_expr(expression, "invalid to_int argument")),
            },
            "to_float" => match &values[0] {
                Value::Float(value) => Ok(Value::Float(*value)),
                Value::Int(value) => Ok(Value::Float(*value as f64)),
                Value::String(value) => value
                    .parse::<f64>()
                    .ok()
                    .filter(|number| number.is_finite())
                    .map(Value::Float)
                    .ok_or_else(|| {
                        runtime_expr(expression, format!("'{value}' is not a valid float"))
                    }),
                _ => Err(runtime_expr(expression, "invalid to_float argument")),
            },
            "upper" => string_unary(&values[0], expression, |text| text.to_uppercase()),
            "lower" => string_unary(&values[0], expression, |text| text.to_lowercase()),
            "trim" => string_unary(&values[0], expression, |text| text.trim().to_owned()),
            "trim_start" => {
                string_unary(&values[0], expression, |text| text.trim_start().to_owned())
            }
            "trim_end" => string_unary(&values[0], expression, |text| text.trim_end().to_owned()),
            "contains" => match (&values[0], &values[1]) {
                (Value::String(text), Value::String(part)) => Ok(Value::Bool(text.contains(part))),
                (Value::Array(items), value) => Ok(Value::Bool(items.contains(value))),
                _ => Err(runtime_expr(expression, "invalid contains arguments")),
            },
            "starts_with" => match (&values[0], &values[1]) {
                (Value::String(text), Value::String(part)) => {
                    Ok(Value::Bool(text.starts_with(part)))
                }
                _ => Err(runtime_expr(expression, "starts_with expects strings")),
            },
            "ends_with" => match (&values[0], &values[1]) {
                (Value::String(text), Value::String(part)) => Ok(Value::Bool(text.ends_with(part))),
                _ => Err(runtime_expr(expression, "ends_with expects strings")),
            },
            "replace" => match (&values[0], &values[1], &values[2]) {
                (Value::String(text), Value::String(from), Value::String(to)) => {
                    Ok(Value::String(text.replace(from, to)))
                }
                _ => Err(runtime_expr(expression, "replace expects string arguments")),
            },
            "split" => match (&values[0], &values[1]) {
                (Value::String(text), Value::String(separator)) => {
                    if separator.is_empty() {
                        return Err(runtime_expr(expression, "split separator cannot be empty"));
                    }
                    Ok(Value::Array(
                        text.split(separator)
                            .map(|part| Value::String(part.to_owned()))
                            .collect(),
                    ))
                }
                _ => Err(runtime_expr(expression, "split expects string arguments")),
            },
            "join" => match (&values[0], &values[1]) {
                (Value::String(separator), Value::Array(items)) => {
                    let mut parts = Vec::with_capacity(items.len());
                    for item in items {
                        match item {
                            Value::String(text) => parts.push(text.clone()),
                            _ => return Err(runtime_expr(expression, "join expects string array")),
                        }
                    }
                    Ok(Value::String(parts.join(separator)))
                }
                _ => Err(runtime_expr(expression, "invalid join arguments")),
            },
            "repeat" => match (&values[0], &values[1]) {
                (Value::String(text), Value::Int(count)) => {
                    if *count < 0 {
                        return Err(runtime_expr(expression, "repeat count cannot be negative"));
                    }
                    let count = usize::try_from(*count)
                        .map_err(|_| runtime_expr(expression, "repeat count is too large"))?;
                    if count.saturating_mul(text.len()) > 1_000_000 {
                        return Err(runtime_expr(
                            expression,
                            "repeated text exceeds 1,000,000 bytes",
                        ));
                    }
                    Ok(Value::String(text.repeat(count)))
                }
                _ => Err(runtime_expr(expression, "repeat expects string and int")),
            },
            "find" => string_search("find", &values, expression),
            "count" => string_search("count", &values, expression),
            "slice" => slice_value(&values, expression),
            "abs" => match &values[0] {
                Value::Int(value) => value
                    .checked_abs()
                    .map(Value::Int)
                    .ok_or_else(|| runtime_expr(expression, "integer overflow in abs")),
                Value::Float(value) => Ok(Value::Float(value.abs())),
                _ => Err(runtime_expr(expression, "abs expects a number")),
            },
            "min" | "max" => numeric_min_max(callee, &values, expression),
            "mean" => stats_mean(&values, expression),
            "median" => stats_median(&values, expression),
            "variance" => stats_variance(&values, expression),
            "stdev" => stats_stdev(&values, expression),
            "clamp" => numeric_clamp(&values, expression),
            "pow" => float_binary(&values, expression, |left, right| left.powf(right)),
            "sqrt" => match values[0] {
                Value::Float(value) if value >= 0.0 => Ok(Value::Float(value.sqrt())),
                Value::Float(_) => Err(runtime_expr(
                    expression,
                    "sqrt expects a non-negative value",
                )),
                _ => Err(runtime_expr(expression, "sqrt expects a float")),
            },
            "round" | "floor" | "ceil" => {
                let Value::Float(value) = values[0] else {
                    return Err(runtime_expr(expression, "rounding expects a float"));
                };
                let rounded = match callee {
                    "round" => value.round(),
                    "floor" => value.floor(),
                    "ceil" => value.ceil(),
                    _ => unreachable!(),
                };
                float_to_int(rounded, expression)
            }
            "reverse" => match &values[0] {
                Value::String(value) => Ok(Value::String(value.chars().rev().collect())),
                Value::Array(items) => Ok(Value::Array(items.iter().rev().cloned().collect())),
                _ => Err(runtime_expr(
                    expression,
                    "reverse expects a string or array",
                )),
            },
            "sort" => sort_array(&values[0], expression),
            "sum" => sum_array(&values[0], expression),
            "insert" => array_insert(&values, expression),
            "remove" => array_remove(&values, expression),
            "range" => self.range(values, expression),
            "append" => {
                let Value::Array(mut array) = values[0].clone() else {
                    return Err(runtime_expr(expression, "invalid append argument"));
                };
                array.push(values[1].clone());
                Ok(Value::Array(array))
            }
            _ => self.call_user_function(callee, values, expression),
        }
    }

    pub(super) fn read_line(
        &mut self,
        prompt: Option<&Value>,
        expression: &Expr,
    ) -> Result<Value, Diagnostic> {
        if let Some(prompt) = prompt {
            self.emit(&prompt.to_string())?;
        }
        let value = match &mut self.input {
            Input::Disabled => {
                return Err(runtime_expr(
                    expression,
                    "input is disabled in this runtime context",
                ));
            }
            Input::Lines(lines) => lines
                .pop_front()
                .ok_or_else(|| runtime_expr(expression, "no more input lines are available"))?,
            Input::Stdin => {
                let mut line = String::new();
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                let read = handle.read_line(&mut line).map_err(|error| {
                    runtime_expr(expression, format!("could not read input: {error}"))
                })?;
                if read == 0 {
                    return Err(runtime_expr(expression, "input reached end of file"));
                }
                line
            }
        };
        Ok(Value::String(
            value.trim_end_matches(['\r', '\n']).to_owned(),
        ))
    }

    pub(super) fn range(&self, values: Vec<Value>, expression: &Expr) -> Result<Value, Diagnostic> {
        let numbers = values
            .into_iter()
            .map(|value| match value {
                Value::Int(value) => Ok(value),
                _ => Err(runtime_expr(expression, "range arguments must be int")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (mut current, end, step) = match numbers.as_slice() {
            [end] => (0, *end, 1),
            [start, end] => (*start, *end, 1),
            [start, end, step] => (*start, *end, *step),
            _ => return Err(runtime_expr(expression, "range expects 1 to 3 arguments")),
        };
        if step == 0 {
            return Err(runtime_expr(expression, "range step cannot be zero"));
        }
        let mut result = Vec::new();
        while (step > 0 && current < end) || (step < 0 && current > end) {
            if result.len() >= 1_000_000 {
                return Err(runtime_expr(
                    expression,
                    "range cannot contain more than 1,000,000 values",
                ));
            }
            result.push(Value::Int(current));
            current = current
                .checked_add(step)
                .ok_or_else(|| runtime_expr(expression, "integer overflow in range"))?;
        }
        Ok(Value::Array(result))
    }
}

pub(super) fn float_to_int(value: f64, expression: &Expr) -> Result<Value, Diagnostic> {
    if value.is_finite() && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
        Ok(Value::Int(value as i64))
    } else {
        Err(runtime_expr(
            expression,
            "float value cannot be represented as int",
        ))
    }
}

fn string_unary(
    value: &Value,
    expression: &Expr,
    transform: impl FnOnce(&str) -> String,
) -> Result<Value, Diagnostic> {
    match value {
        Value::String(text) => Ok(Value::String(transform(text))),
        _ => Err(runtime_expr(expression, "expected a string value")),
    }
}

fn string_search(name: &str, values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    match (&values[0], &values[1]) {
        (Value::String(text), Value::String(part)) => {
            if name == "find" {
                let index = text
                    .find(part)
                    .map_or(-1, |byte_index| text[..byte_index].chars().count() as i64);
                Ok(Value::Int(index))
            } else {
                Ok(Value::Int(text.matches(part).count() as i64))
            }
        }
        _ => Err(runtime_expr(expression, format!("{name} expects strings"))),
    }
}

fn normalized_bound(value: i64, length: usize) -> usize {
    if value < 0 {
        let positive = (-value) as usize;
        length.saturating_sub(positive)
    } else {
        usize::try_from(value).unwrap_or(usize::MAX).min(length)
    }
}

fn slice_bounds(
    values: &[Value],
    length: usize,
    expression: &Expr,
) -> Result<(usize, usize), Diagnostic> {
    let start = match &values[1] {
        Value::Int(value) => normalized_bound(*value, length),
        _ => return Err(runtime_expr(expression, "slice start must be an int")),
    };
    let end = match values.get(2) {
        Some(Value::Int(value)) => normalized_bound(*value, length),
        None => length,
        _ => return Err(runtime_expr(expression, "slice end must be an int")),
    };
    if start > end {
        Ok((start, start))
    } else {
        Ok((start, end))
    }
}

fn slice_value(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    match &values[0] {
        Value::String(text) => {
            let characters = text.chars().collect::<Vec<_>>();
            let (start, end) = slice_bounds(values, characters.len(), expression)?;
            let sliced = characters[start..end].iter().collect::<String>();
            Ok(Value::String(sliced))
        }
        Value::Array(items) => {
            let (start, end) = slice_bounds(values, items.len(), expression)?;
            Ok(Value::Array(items[start..end].to_vec()))
        }
        _ => Err(runtime_expr(expression, "slice expects a string or array")),
    }
}

fn numeric_min_max(name: &str, values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    if values.len() == 1 {
        let Value::Array(items) = &values[0] else {
            return Err(runtime_expr(expression, format!("{name} expects an array")));
        };
        let Some(first) = items.first() else {
            return Err(runtime_expr(
                expression,
                format!("{name} cannot be called on an empty array"),
            ));
        };
        match first {
            Value::Int(first_val) => {
                let mut best = *first_val;
                for item in &items[1..] {
                    let Value::Int(value) = item else {
                        return Err(runtime_expr(
                            expression,
                            format!("{name} array types do not match"),
                        ));
                    };
                    best = if name == "min" {
                        best.min(*value)
                    } else {
                        best.max(*value)
                    };
                }
                Ok(Value::Int(best))
            }
            Value::Float(first_val) => {
                let mut best = *first_val;
                for item in &items[1..] {
                    let Value::Float(value) = item else {
                        return Err(runtime_expr(
                            expression,
                            format!("{name} array types do not match"),
                        ));
                    };
                    best = if name == "min" {
                        best.min(*value)
                    } else {
                        best.max(*value)
                    };
                }
                Ok(Value::Float(best))
            }
            Value::String(first_val) => {
                let mut best = first_val.clone();
                let mut best_len = best.chars().count();
                for item in &items[1..] {
                    let Value::String(value) = item else {
                        return Err(runtime_expr(
                            expression,
                            format!("{name} array types do not match"),
                        ));
                    };
                    let len = value.chars().count();
                    let should_replace = if name == "min" {
                        len < best_len || (len == best_len && value < &best)
                    } else {
                        len > best_len || (len == best_len && value > &best)
                    };
                    if should_replace {
                        best = value.clone();
                        best_len = len;
                    }
                }
                Ok(Value::String(best))
            }
            _ => Err(runtime_expr(
                expression,
                format!("unsupported {name} element type"),
            )),
        }
    } else if values.len() == 2 {
        match (&values[0], &values[1]) {
            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(if name == "min" {
                (*left).min(*right)
            } else {
                (*left).max(*right)
            })),
            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(if name == "min" {
                left.min(*right)
            } else {
                left.max(*right)
            })),
            _ => Err(runtime_expr(expression, "min/max expect matching numbers")),
        }
    } else {
        Err(runtime_expr(
            expression,
            format!("{name} expects 1 or 2 arguments"),
        ))
    }
}

fn extract_numeric_slice(
    values: &[Value],
    name: &str,
    expression: &Expr,
) -> Result<Vec<f64>, Diagnostic> {
    let Some(Value::Array(items)) = values.first() else {
        return Err(runtime_expr(expression, format!("{name} expects an array")));
    };
    if items.is_empty() {
        return Err(runtime_expr(
            expression,
            format!("{name} cannot be called on an empty array"),
        ));
    }
    let mut numbers = Vec::with_capacity(items.len());
    for item in items {
        match item {
            Value::Int(value) => numbers.push(*value as f64),
            Value::Float(value) => numbers.push(*value),
            _ => {
                return Err(runtime_expr(
                    expression,
                    format!("{name} expects a numeric array"),
                ));
            }
        }
    }
    Ok(numbers)
}

fn stats_mean(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let numbers = extract_numeric_slice(values, "mean", expression)?;
    let total: f64 = numbers.iter().sum();
    Ok(Value::Float(total / numbers.len() as f64))
}

fn stats_median(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let mut numbers = extract_numeric_slice(values, "median", expression)?;
    numbers.sort_by(|left, right| left.total_cmp(right));
    let len = numbers.len();
    let mid = len / 2;
    if len % 2 == 1 {
        Ok(Value::Float(numbers[mid]))
    } else {
        Ok(Value::Float((numbers[mid - 1] + numbers[mid]) / 2.0))
    }
}

fn stats_variance(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let numbers = extract_numeric_slice(values, "variance", expression)?;
    let len = numbers.len() as f64;
    let mean = numbers.iter().sum::<f64>() / len;
    let sum_sq_diff: f64 = numbers.iter().map(|value| (value - mean).powi(2)).sum();
    Ok(Value::Float(sum_sq_diff / len))
}

fn stats_stdev(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let numbers = extract_numeric_slice(values, "stdev", expression)?;
    let len = numbers.len() as f64;
    let mean = numbers.iter().sum::<f64>() / len;
    let sum_sq_diff: f64 = numbers.iter().map(|value| (value - mean).powi(2)).sum();
    let variance = sum_sq_diff / len;
    Ok(Value::Float(variance.max(0.0).sqrt()))
}

fn numeric_clamp(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    match (&values[0], &values[1], &values[2]) {
        (Value::Int(value), Value::Int(low), Value::Int(high)) if low <= high => {
            Ok(Value::Int((*value).clamp(*low, *high)))
        }
        (Value::Float(value), Value::Float(low), Value::Float(high)) if low <= high => {
            Ok(Value::Float((*value).clamp(*low, *high)))
        }
        _ => Err(runtime_expr(
            expression,
            "clamp expects three matching numbers with low <= high",
        )),
    }
}

fn float_binary(
    values: &[Value],
    expression: &Expr,
    operation: impl FnOnce(f64, f64) -> f64,
) -> Result<Value, Diagnostic> {
    match (&values[0], &values[1]) {
        (Value::Float(left), Value::Float(right)) => {
            let result = operation(*left, *right);
            if result.is_finite() {
                Ok(Value::Float(result))
            } else {
                Err(runtime_expr(
                    expression,
                    "floating-point operation produced a non-finite result",
                ))
            }
        }
        _ => Err(runtime_expr(expression, "expected two float arguments")),
    }
}

fn sort_array(value: &Value, expression: &Expr) -> Result<Value, Diagnostic> {
    let Value::Array(items) = value else {
        return Err(runtime_expr(expression, "sort expects an array"));
    };
    let mut result = items.clone();
    if result.iter().all(|item| matches!(item, Value::Int(_))) {
        result.sort_by_key(|item| match item {
            Value::Int(value) => *value,
            _ => 0,
        });
    } else if result.iter().all(|item| matches!(item, Value::Float(_))) {
        result.sort_by(|left, right| match (left, right) {
            (Value::Float(left), Value::Float(right)) => left.total_cmp(right),
            _ => std::cmp::Ordering::Equal,
        });
    } else if result.iter().all(|item| matches!(item, Value::String(_))) {
        result.sort_by(|left, right| match (left, right) {
            (Value::String(left), Value::String(right)) => left.cmp(right),
            _ => std::cmp::Ordering::Equal,
        });
    } else if !result.is_empty() {
        return Err(runtime_expr(expression, "array elements are not sortable"));
    }
    Ok(Value::Array(result))
}

fn sum_array(value: &Value, expression: &Expr) -> Result<Value, Diagnostic> {
    let Value::Array(items) = value else {
        return Err(runtime_expr(expression, "sum expects an array"));
    };
    let Some(first) = items.first() else {
        return Err(runtime_expr(
            expression,
            "sum cannot infer the type of an empty array",
        ));
    };
    match first {
        Value::Int(_) => items
            .iter()
            .try_fold(0_i64, |total, item| match item {
                Value::Int(value) => total
                    .checked_add(*value)
                    .ok_or_else(|| runtime_expr(expression, "integer overflow in sum")),
                _ => Err(runtime_expr(expression, "sum array types do not match")),
            })
            .map(Value::Int),
        Value::Float(_) => items
            .iter()
            .try_fold(0.0, |total, item| match item {
                Value::Float(value) => Ok(total + value),
                _ => Err(runtime_expr(expression, "sum array types do not match")),
            })
            .map(Value::Float),
        _ => Err(runtime_expr(expression, "sum expects a numeric array")),
    }
}

fn array_insert(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let (Value::Array(items), Value::Int(index), value) = (&values[0], &values[1], &values[2])
    else {
        return Err(runtime_expr(expression, "invalid insert arguments"));
    };
    let mut result = items.clone();
    let index = if *index < 0 {
        normalized_bound(*index, result.len()).min(result.len())
    } else {
        usize::try_from(*index)
            .unwrap_or(usize::MAX)
            .min(result.len())
    };
    result.insert(index, value.clone());
    Ok(Value::Array(result))
}

fn array_remove(values: &[Value], expression: &Expr) -> Result<Value, Diagnostic> {
    let (Value::Array(items), Value::Int(index)) = (&values[0], &values[1]) else {
        return Err(runtime_expr(expression, "invalid remove arguments"));
    };
    if items.is_empty() {
        return Err(runtime_expr(
            expression,
            "cannot remove from an empty array",
        ));
    }
    let index = if *index < 0 {
        normalized_bound(*index, items.len())
    } else {
        usize::try_from(*index).map_err(|_| runtime_expr(expression, "array index is too large"))?
    };
    if index >= items.len() {
        return Err(runtime_expr(
            expression,
            format!("array index out of bounds: {index}"),
        ));
    }
    let mut result = items.clone();
    result.remove(index);
    Ok(Value::Array(result))
}
