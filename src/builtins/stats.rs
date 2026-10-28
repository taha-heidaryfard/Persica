use super::{Builtin, ParamType, ReturnType, runtime_error};
use crate::runtime::Value;
use std::collections::HashMap;

pub fn register() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "product_int",
            params: &[ParamType::IntArray],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "product_int cannot be called on an empty array",
                    ));
                }
                let mut prod: i64 = 1;
                for item in items {
                    if let Value::Int(v) = item {
                        prod = prod.checked_mul(*v).ok_or_else(|| {
                            runtime_error(expr, "integer overflow in product_int")
                        })?;
                    }
                }
                Ok(Value::Int(prod))
            },
        },
        Builtin {
            name: "product_float",
            params: &[ParamType::FloatArray],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "product_float cannot be called on an empty array",
                    ));
                }
                let mut prod = 1.0;
                for item in items {
                    if let Value::Float(v) = item {
                        prod *= *v;
                    }
                }
                if !prod.is_finite() {
                    return Err(runtime_error(
                        expr,
                        "product_float produced a non-finite result",
                    ));
                }
                Ok(Value::Float(prod))
            },
        },
        Builtin {
            name: "geometric_mean",
            params: &[ParamType::FloatArray],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "geometric_mean cannot be called on an empty array",
                    ));
                }
                let mut log_sum = 0.0;
                for item in items {
                    if let Value::Float(v) = item {
                        if *v <= 0.0 {
                            return Err(runtime_error(
                                expr,
                                "geometric_mean expects strictly positive numbers",
                            ));
                        }
                        log_sum += v.ln();
                    }
                }
                let geo_mean = (log_sum / items.len() as f64).exp();
                Ok(Value::Float(geo_mean))
            },
        },
        Builtin {
            name: "harmonic_mean",
            params: &[ParamType::FloatArray],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "harmonic_mean cannot be called on an empty array",
                    ));
                }
                let mut reciprocal_sum = 0.0;
                for item in items {
                    if let Value::Float(v) = item {
                        if *v <= 0.0 {
                            return Err(runtime_error(
                                expr,
                                "harmonic_mean expects strictly positive numbers",
                            ));
                        }
                        reciprocal_sum += 1.0 / *v;
                    }
                }
                let harm_mean = items.len() as f64 / reciprocal_sum;
                Ok(Value::Float(harm_mean))
            },
        },
        Builtin {
            name: "mode_int",
            params: &[ParamType::IntArray],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "mode_int cannot be called on an empty array",
                    ));
                }
                let mut counts = HashMap::new();
                for item in items {
                    if let Value::Int(v) = item {
                        *counts.entry(*v).or_insert(0usize) += 1;
                    }
                }
                let mut best_val = 0;
                let mut max_count = 0;
                for (val, count) in counts {
                    if count > max_count {
                        max_count = count;
                        best_val = val;
                    }
                }
                Ok(Value::Int(best_val))
            },
        },
        Builtin {
            name: "lerp",
            params: &[ParamType::Float, ParamType::Float, ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(a), Value::Float(b), Value::Float(t)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                Ok(Value::Float(a + t * (b - a)))
            },
        },
        Builtin {
            name: "normalize",
            params: &[ParamType::FloatArray],
            return_type: ReturnType::FloatArray,
            handler: |values, expr| {
                let Value::Array(items) = &values[0] else {
                    unreachable!()
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        expr,
                        "normalize cannot be called on an empty array",
                    ));
                }
                let mut min_val = f64::INFINITY;
                let mut max_val = f64::NEG_INFINITY;
                for item in items {
                    if let Value::Float(v) = item {
                        min_val = min_val.min(*v);
                        max_val = max_val.max(*v);
                    }
                }
                let range = max_val - min_val;
                let result = if range.abs() < f64::EPSILON {
                    vec![Value::Float(0.0); items.len()]
                } else {
                    items
                        .iter()
                        .map(|item| {
                            if let Value::Float(v) = item {
                                Value::Float((v - min_val) / range)
                            } else {
                                unreachable!()
                            }
                        })
                        .collect()
                };
                Ok(Value::Array(result))
            },
        },
        Builtin {
            name: "z_score",
            params: &[ParamType::Float, ParamType::Float, ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let (Value::Float(val), Value::Float(mean), Value::Float(stdev)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                if *stdev <= 0.0 {
                    return Err(runtime_error(
                        expr,
                        "z_score expects positive standard deviation",
                    ));
                }
                Ok(Value::Float((val - mean) / stdev))
            },
        },
        Builtin {
            name: "distance_2d",
            params: &[
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
            ],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(x1), Value::Float(y1), Value::Float(x2), Value::Float(y2)) =
                    (&values[0], &values[1], &values[2], &values[3])
                else {
                    unreachable!()
                };
                let dx = x2 - x1;
                let dy = y2 - y1;
                Ok(Value::Float(dx.hypot(dy)))
            },
        },
        Builtin {
            name: "distance_3d",
            params: &[
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
                ParamType::Float,
            ],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(x1), Value::Float(y1), Value::Float(z1)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                let (Value::Float(x2), Value::Float(y2), Value::Float(z2)) =
                    (&values[3], &values[4], &values[5])
                else {
                    unreachable!()
                };
                let dx = x2 - x1;
                let dy = y2 - y1;
                let dz = z2 - z1;
                Ok(Value::Float((dx * dx + dy * dy + dz * dz).sqrt()))
            },
        },
    ]
}
