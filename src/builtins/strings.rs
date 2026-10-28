use super::{Builtin, ParamType, ReturnType, runtime_error};
use crate::runtime::Value;

pub fn register() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "capitalize",
            params: &[ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let mut chars = s.chars();
                let result = match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                };
                Ok(Value::String(result))
            },
        },
        Builtin {
            name: "title_case",
            params: &[ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let mut result = String::new();
                let mut capitalize_next = true;
                for ch in s.chars() {
                    if ch.is_whitespace() || ch == '_' || ch == '-' {
                        capitalize_next = true;
                        result.push(ch);
                    } else if capitalize_next {
                        result.extend(ch.to_uppercase());
                        capitalize_next = false;
                    } else {
                        result.extend(ch.to_lowercase());
                    }
                }
                Ok(Value::String(result))
            },
        },
        Builtin {
            name: "is_empty",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(s.is_empty()))
            },
        },
        Builtin {
            name: "is_ascii",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(s.is_ascii()))
            },
        },
        Builtin {
            name: "is_numeric",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(
                    !s.is_empty() && s.chars().all(|c| c.is_numeric()),
                ))
            },
        },
        Builtin {
            name: "is_alphabetic",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(
                    !s.is_empty() && s.chars().all(|c| c.is_alphabetic()),
                ))
            },
        },
        Builtin {
            name: "is_alphanumeric",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(
                    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric()),
                ))
            },
        },
        Builtin {
            name: "is_whitespace",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(
                    !s.is_empty() && s.chars().all(|c| c.is_whitespace()),
                ))
            },
        },
        Builtin {
            name: "is_upper",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let has_alphabetic = s.chars().any(|c| c.is_alphabetic());
                let all_upper = s
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .all(|c| c.is_uppercase());
                Ok(Value::Bool(has_alphabetic && all_upper))
            },
        },
        Builtin {
            name: "is_lower",
            params: &[ParamType::String],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let has_alphabetic = s.chars().any(|c| c.is_alphabetic());
                let all_lower = s
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .all(|c| c.is_lowercase());
                Ok(Value::Bool(has_alphabetic && all_lower))
            },
        },
        Builtin {
            name: "pad_left",
            params: &[ParamType::String, ParamType::Int, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, expr| {
                let (Value::String(s), Value::Int(target_len), Value::String(fill)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *target_len < 0 {
                    return Err(runtime_error(
                        expr,
                        "pad_left target length cannot be negative",
                    ));
                }
                let target_len = *target_len as usize;
                let current_len = s.chars().count();
                if current_len >= target_len {
                    return Ok(Value::String(s.clone()));
                }
                if fill.is_empty() {
                    return Err(runtime_error(expr, "fill string cannot be empty"));
                }
                let needed = target_len - current_len;
                let fill_chars: Vec<char> = fill.chars().collect();
                let mut padding = String::with_capacity(needed);
                for i in 0..needed {
                    padding.push(fill_chars[i % fill_chars.len()]);
                }
                padding.push_str(s);
                Ok(Value::String(padding))
            },
        },
        Builtin {
            name: "pad_right",
            params: &[ParamType::String, ParamType::Int, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, expr| {
                let (Value::String(s), Value::Int(target_len), Value::String(fill)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *target_len < 0 {
                    return Err(runtime_error(
                        expr,
                        "pad_right target length cannot be negative",
                    ));
                }
                let target_len = *target_len as usize;
                let current_len = s.chars().count();
                if current_len >= target_len {
                    return Ok(Value::String(s.clone()));
                }
                if fill.is_empty() {
                    return Err(runtime_error(expr, "fill string cannot be empty"));
                }
                let needed = target_len - current_len;
                let fill_chars: Vec<char> = fill.chars().collect();
                let mut result = s.clone();
                for i in 0..needed {
                    result.push(fill_chars[i % fill_chars.len()]);
                }
                Ok(Value::String(result))
            },
        },
        Builtin {
            name: "center",
            params: &[ParamType::String, ParamType::Int, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, expr| {
                let (Value::String(s), Value::Int(target_len), Value::String(fill)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *target_len < 0 {
                    return Err(runtime_error(
                        expr,
                        "center target length cannot be negative",
                    ));
                }
                let target_len = *target_len as usize;
                let current_len = s.chars().count();
                if current_len >= target_len {
                    return Ok(Value::String(s.clone()));
                }
                if fill.is_empty() {
                    return Err(runtime_error(expr, "fill string cannot be empty"));
                }
                let needed = target_len - current_len;
                let left_pad = needed / 2;
                let right_pad = needed - left_pad;
                let fill_chars: Vec<char> = fill.chars().collect();
                let mut result = String::new();
                for i in 0..left_pad {
                    result.push(fill_chars[i % fill_chars.len()]);
                }
                result.push_str(s);
                for i in 0..right_pad {
                    result.push(fill_chars[i % fill_chars.len()]);
                }
                Ok(Value::String(result))
            },
        },
        Builtin {
            name: "char_at",
            params: &[ParamType::String, ParamType::Int],
            return_type: ReturnType::String,
            handler: |values, expr| {
                let (Value::String(s), Value::Int(index)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *index < 0 {
                    return Err(runtime_error(expr, "char_at index cannot be negative"));
                }
                let ch = s.chars().nth(*index as usize).ok_or_else(|| {
                    runtime_error(expr, format!("char_at index {index} out of bounds"))
                })?;
                Ok(Value::String(ch.to_string()))
            },
        },
        Builtin {
            name: "code_point",
            params: &[ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let ch = s.chars().next().ok_or_else(|| {
                    runtime_error(expr, "code_point cannot be called on an empty string")
                })?;
                Ok(Value::Int(ch as i64))
            },
        },
        Builtin {
            name: "from_code_point",
            params: &[ParamType::Int],
            return_type: ReturnType::String,
            handler: |values, expr| {
                let Value::Int(code) = values[0] else {
                    unreachable!()
                };
                if !(0..=0x10FFFF).contains(&code) {
                    return Err(runtime_error(expr, "invalid Unicode code point"));
                }
                let ch = char::from_u32(code as u32)
                    .ok_or_else(|| runtime_error(expr, "invalid Unicode scalar value"))?;
                Ok(Value::String(ch.to_string()))
            },
        },
        Builtin {
            name: "chars",
            params: &[ParamType::String],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let items = s.chars().map(|c| Value::String(c.to_string())).collect();
                Ok(Value::Array(items))
            },
        },
        Builtin {
            name: "lines",
            params: &[ParamType::String],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let items = s
                    .lines()
                    .map(|line| Value::String(line.to_string()))
                    .collect();
                Ok(Value::Array(items))
            },
        },
        Builtin {
            name: "words",
            params: &[ParamType::String],
            return_type: ReturnType::StringArray,
            handler: |values, _expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let items = s
                    .split_whitespace()
                    .map(|w| Value::String(w.to_string()))
                    .collect();
                Ok(Value::Array(items))
            },
        },
        Builtin {
            name: "remove_prefix",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let (Value::String(s), Value::String(prefix)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let result = s.strip_prefix(prefix.as_str()).unwrap_or(s);
                Ok(Value::String(result.to_string()))
            },
        },
        Builtin {
            name: "remove_suffix",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let (Value::String(s), Value::String(suffix)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let result = s.strip_suffix(suffix.as_str()).unwrap_or(s);
                Ok(Value::String(result.to_string()))
            },
        },
        Builtin {
            name: "levenshtein",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::String(s1), Value::String(s2)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let c1: Vec<char> = s1.chars().collect();
                let c2: Vec<char> = s2.chars().collect();
                let n = c1.len();
                let m = c2.len();
                let mut dp = vec![vec![0; m + 1]; n + 1];
                for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
                    row[0] = i;
                }
                for (j, cell) in dp[0].iter_mut().enumerate().take(m + 1) {
                    *cell = j;
                }
                for i in 1..=n {
                    for j in 1..=m {
                        let cost = if c1[i - 1] == c2[j - 1] { 0 } else { 1 };
                        dp[i][j] = (dp[i - 1][j] + 1)
                            .min(dp[i][j - 1] + 1)
                            .min(dp[i - 1][j - 1] + cost);
                    }
                }
                Ok(Value::Int(dp[n][m] as i64))
            },
        },
        Builtin {
            name: "to_hex",
            params: &[ParamType::Int],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                Ok(Value::String(format!("{:x}", n)))
            },
        },
        Builtin {
            name: "to_bin",
            params: &[ParamType::Int],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                Ok(Value::String(format!("{:b}", n)))
            },
        },
        Builtin {
            name: "to_oct",
            params: &[ParamType::Int],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                Ok(Value::String(format!("{:o}", n)))
            },
        },
        Builtin {
            name: "from_hex",
            params: &[ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let trimmed = s.trim_start_matches("0x").trim_start_matches("0X");
                let n = i64::from_str_radix(trimmed, 16).map_err(|_| {
                    runtime_error(expr, format!("invalid hexadecimal string '{s}'"))
                })?;
                Ok(Value::Int(n))
            },
        },
        Builtin {
            name: "from_bin",
            params: &[ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let trimmed = s.trim_start_matches("0b").trim_start_matches("0B");
                let n = i64::from_str_radix(trimmed, 2)
                    .map_err(|_| runtime_error(expr, format!("invalid binary string '{s}'")))?;
                Ok(Value::Int(n))
            },
        },
        Builtin {
            name: "from_oct",
            params: &[ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::String(s) = &values[0] else {
                    unreachable!()
                };
                let trimmed = s.trim_start_matches("0o").trim_start_matches("0O");
                let n = i64::from_str_radix(trimmed, 8)
                    .map_err(|_| runtime_error(expr, format!("invalid octal string '{s}'")))?;
                Ok(Value::Int(n))
            },
        },
        Builtin {
            name: "common_prefix",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let (Value::String(s1), Value::String(s2)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut prefix = String::new();
                for (c1, c2) in s1.chars().zip(s2.chars()) {
                    if c1 == c2 {
                        prefix.push(c1);
                    } else {
                        break;
                    }
                }
                Ok(Value::String(prefix))
            },
        },
        Builtin {
            name: "common_suffix",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::String,
            handler: |values, _expr| {
                let (Value::String(s1), Value::String(s2)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut suffix = Vec::new();
                for (c1, c2) in s1.chars().rev().zip(s2.chars().rev()) {
                    if c1 == c2 {
                        suffix.push(c1);
                    } else {
                        break;
                    }
                }
                suffix.reverse();
                Ok(Value::String(suffix.into_iter().collect()))
            },
        },
        Builtin {
            name: "count_char",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let (Value::String(s), Value::String(ch)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let target = ch.chars().next().ok_or_else(|| {
                    runtime_error(expr, "count_char target character cannot be empty")
                })?;
                let count = s.chars().filter(|c| *c == target).count();
                Ok(Value::Int(count as i64))
            },
        },
        Builtin {
            name: "index_of_char",
            params: &[ParamType::String, ParamType::String],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let (Value::String(s), Value::String(ch)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let target = ch.chars().next().ok_or_else(|| {
                    runtime_error(expr, "index_of_char target character cannot be empty")
                })?;
                for (i, c) in s.chars().enumerate() {
                    if c == target {
                        return Ok(Value::Int(i as i64));
                    }
                }
                Ok(Value::Int(-1))
            },
        },
    ]
}
