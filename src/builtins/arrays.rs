use super::{Builtin, ParamType, ReturnType, runtime_error};
use crate::runtime::Value;

pub fn register() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "first",
            params: &[ParamType::AnyArray],
            return_type: ReturnType::ElementOfArg(0),
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                items
                    .first()
                    .cloned()
                    .ok_or_else(|| runtime_error(expr, "first cannot be called on an empty array"))
            },
        },
        Builtin {
            name: "last",
            params: &[ParamType::AnyArray],
            return_type: ReturnType::ElementOfArg(0),
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                items
                    .last()
                    .cloned()
                    .ok_or_else(|| runtime_error(expr, "last cannot be called on an empty array"))
            },
        },
        Builtin {
            name: "head",
            params: &[ParamType::AnyArray],
            return_type: ReturnType::SameAsArg(0),
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "head cannot be called on an empty array",
                    ));
                }
                Ok(Value::Array(items[..items.len() - 1].to_vec()))
            },
        },
        Builtin {
            name: "tail",
            params: &[ParamType::AnyArray],
            return_type: ReturnType::SameAsArg(0),
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "tail cannot be called on an empty array",
                    ));
                }
                Ok(Value::Array(items[1..].to_vec()))
            },
        },
        Builtin {
            name: "take",
            params: &[ParamType::AnyArray, ParamType::Int],
            return_type: ReturnType::SameAsArg(0),
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(count)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 {
                    return Err(runtime_error(expr, "take count cannot be negative"));
                }
                let take_len = (*count as usize).min(items.len());
                Ok(Value::Array(items[..take_len].to_vec()))
            },
        },
        Builtin {
            name: "drop",
            params: &[ParamType::AnyArray, ParamType::Int],
            return_type: ReturnType::SameAsArg(0),
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(count)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 {
                    return Err(runtime_error(expr, "drop count cannot be negative"));
                }
                let drop_len = (*count as usize).min(items.len());
                Ok(Value::Array(items[drop_len..].to_vec()))
            },
        },
        Builtin {
            name: "unique_int",
            params: &[ParamType::IntArray],
            return_type: ReturnType::IntArray,
            handler: |values, _expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                let mut result = Vec::new();
                for item in items {
                    if !result.contains(item) {
                        result.push(item.clone());
                    }
                }
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "unique_float",
            params: &[ParamType::FloatArray],
            return_type: ReturnType::FloatArray,
            handler: |values, _expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                let mut result = Vec::new();
                for item in items {
                    if !result.contains(item) {
                        result.push(item.clone());
                    }
                }
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "unique_string",
            params: &[ParamType::StringArray],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                let mut result = Vec::new();
                for item in items {
                    if !result.contains(item) {
                        result.push(item.clone());
                    }
                }
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "compact_string",
            params: &[ParamType::StringArray],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                let result = items
                    .iter()
                    .filter(|item| match item {
                        Value::String(s) => !s.is_empty(),
                        _ => true,
                    })
                    .cloned()
                    .collect();
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "flatten_int",
            params: &[ParamType::Int2DArray],
            return_type: ReturnType::IntArray,
            handler: |values, _expr| {
                let Value::Array(outer) = &values[0] else {
                    unreachable!()
                };
                let mut flat = Vec::new();
                for inner in outer {
                    if let Value::Array(items) = inner {
                        flat.extend(items.clone());
                    }
                }
                Ok(Value::Array(flat))
            },
        },
        Builtin {
            name: "flatten_float",
            params: &[ParamType::Float2DArray],
            return_type: ReturnType::FloatArray,
            handler: |values, _expr| {
                let Value::Array(outer) = &values[0] else {
                    unreachable!()
                };
                let mut flat = Vec::new();
                for inner in outer {
                    if let Value::Array(items) = inner {
                        flat.extend(items.clone());
                    }
                }
                Ok(Value::Array(flat))
            },
        },
        Builtin {
            name: "flatten_string",
            params: &[ParamType::String2DArray],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::Array(outer) = &values[0] else {
                    unreachable!()
                };
                let mut flat = Vec::new();
                for inner in outer {
                    if let Value::Array(items) = inner {
                        flat.extend(items.clone());
                    }
                }
                Ok(Value::Array(flat))
            },
        },
        Builtin {
            name: "chunk_int",
            params: &[ParamType::IntArray, ParamType::Int],
            return_type: ReturnType::Int2DArray,
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(size)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *size <= 0 {
                    return Err(runtime_error(expr, "chunk size must be greater than zero"));
                }
                let size = *size as usize;
                let chunks: Vec<Value> = items
                    .chunks(size)
                    .map(|c| Value::Array(c.to_vec()))
                    .collect();
                Ok(Value::Array(chunks))
            },
        },
        Builtin {
            name: "chunk_string",
            params: &[ParamType::StringArray, ParamType::Int],
            return_type: ReturnType::String2DArray,
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(size)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *size <= 0 {
                    return Err(runtime_error(expr, "chunk size must be greater than zero"));
                }
                let size = *size as usize;
                let chunks: Vec<Value> = items
                    .chunks(size)
                    .map(|c| Value::Array(c.to_vec()))
                    .collect();
                Ok(Value::Array(chunks))
            },
        },
        Builtin {
            name: "fill_int",
            params: &[ParamType::Int, ParamType::Int],
            return_type: ReturnType::IntArray,
            handler: |values, expr| {
                let (Value::Int(count), Value::Int(val)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 || *count > 1_000_000 {
                    return Err(runtime_error(
                        expr,
                        "fill_int count must be between 0 and 1,000,000",
                    ));
                }
                Ok(Value::Array(vec![Value::Int(*val); *count as usize]))
            },
        },
        Builtin {
            name: "fill_float",
            params: &[ParamType::Int, ParamType::Float],
            return_type: ReturnType::FloatArray,
            handler: |values, expr| {
                let (Value::Int(count), Value::Float(val)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 || *count > 1_000_000 {
                    return Err(runtime_error(
                        expr,
                        "fill_float count must be between 0 and 1,000,000",
                    ));
                }
                Ok(Value::Array(vec![Value::Float(*val); *count as usize]))
            },
        },
        Builtin {
            name: "fill_string",
            params: &[ParamType::Int, ParamType::String],
            return_type: ReturnType::StringArray,
            handler: |values, expr| {
                let (Value::Int(count), Value::String(val)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 || *count > 1_000_000 {
                    return Err(runtime_error(
                        expr,
                        "fill_string count must be between 0 and 1,000,000",
                    ));
                }
                Ok(Value::Array(vec![
                    Value::String(val.clone());
                    *count as usize
                ]))
            },
        },
        Builtin {
            name: "fill_bool",
            params: &[ParamType::Int, ParamType::Bool],
            return_type: ReturnType::BoolArray,
            handler: |values, expr| {
                let (Value::Int(count), Value::Bool(val)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *count < 0 || *count > 1_000_000 {
                    return Err(runtime_error(
                        expr,
                        "fill_bool count must be between 0 and 1,000,000",
                    ));
                }
                Ok(Value::Array(vec![Value::Bool(*val); *count as usize]))
            },
        },
        Builtin {
            name: "index_of_int",
            params: &[ParamType::IntArray, ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::Int(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                for (i, item) in items.iter().enumerate() {
                    if let Value::Int(val) = item
                        && val == target
                    {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
        Builtin {
            name: "index_of_float",
            params: &[ParamType::FloatArray, ParamType::Float],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::Float(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                for (i, item) in items.iter().enumerate() {
                    if let Value::Float(val) = item
                        && (val - target).abs() < f64::EPSILON
                    {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
        Builtin {
            name: "index_of_string",
            params: &[ParamType::StringArray, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::String(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                for (i, item) in items.iter().enumerate() {
                    if let Value::String(val) = item
                        && val == target
                    {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
        Builtin {
            name: "last_index_of_int",
            params: &[ParamType::IntArray, ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::Int(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                for (i, item) in items.iter().enumerate().rev() {
                    if let Value::Int(val) = item
                        && val == target
                    {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
        Builtin {
            name: "last_index_of_string",
            params: &[ParamType::StringArray, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::String(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                for (i, item) in items.iter().enumerate().rev() {
                    if let Value::String(val) = item
                        && val == target
                    {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
        Builtin {
            name: "count_int",
            params: &[ParamType::IntArray, ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::Int(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let count = items
                    .iter()
                    .filter(|item| matches!(item, Value::Int(v) if v == target))
                    .count();
                Ok(Value::Int(count as i64))
            },
        },
        Builtin {
            name: "count_string",
            params: &[ParamType::StringArray, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Array(items), Value::String(target)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let count = items
                    .iter()
                    .filter(|item| matches!(item, Value::String(v) if v == target))
                    .count();
                Ok(Value::Int(count as i64))
            },
        },
        Builtin {
            name: "concat_int",
            params: &[ParamType::IntArray, ParamType::IntArray],
            return_type: ReturnType::IntArray,
            handler: |values, _expr| {
                let (Value::Array(a), Value::Array(b)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "concat_float",
            params: &[ParamType::FloatArray, ParamType::FloatArray],
            return_type: ReturnType::FloatArray,
            handler: |values, _expr| {
                let (Value::Array(a), Value::Array(b)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "concat_string",
            params: &[ParamType::StringArray, ParamType::StringArray],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let (Value::Array(a), Value::Array(b)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "swap_int",
            params: &[ParamType::IntArray, ParamType::Int, ParamType::Int],
            return_type: ReturnType::IntArray,
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(i), Value::Int(j)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *i < 0 || *j < 0 || *i as usize >= items.len() || *j as usize >= items.len() {
                    return Err(runtime_error(expr, "swap index out of bounds"));
                }
                let mut result = items.clone();
                result.swap(*i as usize, *j as usize);
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "swap_string",
            params: &[ParamType::StringArray, ParamType::Int, ParamType::Int],
            return_type: ReturnType::StringArray,
            handler: |values, expr| {
                let (Value::Array(items), Value::Int(i), Value::Int(j)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *i < 0 || *j < 0 || *i as usize >= items.len() || *j as usize >= items.len() {
                    return Err(runtime_error(expr, "swap index out of bounds"));
                }
                let mut result = items.clone();
                result.swap(*i as usize, *j as usize);
                Ok(Value::Array(result))
            },
        },
    ]
}
