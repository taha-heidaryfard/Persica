use super::{Builtin, ParamType, ReturnType, runtime_error};
use crate::runtime::Value;

pub fn register() -> Vec<Builtin> {
    vec![
        Builtin {
            name: "pi",
            params: &[],
            return_type: ReturnType::Float,
            handler: |_values, _expr| Ok(Value::Float(std::f64::consts::PI)),
        },
        Builtin {
            name: "e",
            params: &[],
            return_type: ReturnType::Float,
            handler: |_values, _expr| Ok(Value::Float(std::f64::consts::E)),
        },
        Builtin {
            name: "tau",
            params: &[],
            return_type: ReturnType::Float,
            handler: |_values, _expr| Ok(Value::Float(std::f64::consts::TAU)),
        },
        Builtin {
            name: "sin",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.sin()))
            },
        },
        Builtin {
            name: "cos",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.cos()))
            },
        },
        Builtin {
            name: "tan",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.tan()))
            },
        },
        Builtin {
            name: "asin",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if !(-1.0..=1.0).contains(&x) {
                    return Err(runtime_error(
                        expr,
                        "asin argument must be between -1.0 and 1.0",
                    ));
                }
                Ok(Value::Float(x.asin()))
            },
        },
        Builtin {
            name: "acos",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if !(-1.0..=1.0).contains(&x) {
                    return Err(runtime_error(
                        expr,
                        "acos argument must be between -1.0 and 1.0",
                    ));
                }
                Ok(Value::Float(x.acos()))
            },
        },
        Builtin {
            name: "atan",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.atan()))
            },
        },
        Builtin {
            name: "atan2",
            params: &[ParamType::Float, ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(y), Value::Float(x)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                Ok(Value::Float(y.atan2(*x)))
            },
        },
        Builtin {
            name: "sinh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.sinh()))
            },
        },
        Builtin {
            name: "cosh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.cosh()))
            },
        },
        Builtin {
            name: "tanh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.tanh()))
            },
        },
        Builtin {
            name: "asinh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.asinh()))
            },
        },
        Builtin {
            name: "acosh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if x < 1.0 {
                    return Err(runtime_error(expr, "acosh argument must be >= 1.0"));
                }
                Ok(Value::Float(x.acosh()))
            },
        },
        Builtin {
            name: "atanh",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if x <= -1.0 || x >= 1.0 {
                    return Err(runtime_error(
                        expr,
                        "atanh argument must be between -1.0 and 1.0",
                    ));
                }
                Ok(Value::Float(x.atanh()))
            },
        },
        Builtin {
            name: "deg_to_rad",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(deg) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(deg.to_radians()))
            },
        },
        Builtin {
            name: "rad_to_deg",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(rad) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(rad.to_degrees()))
            },
        },
        Builtin {
            name: "exp",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.exp()))
            },
        },
        Builtin {
            name: "exp2",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.exp2()))
            },
        },
        Builtin {
            name: "ln",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if x <= 0.0 {
                    return Err(runtime_error(expr, "ln expects positive argument"));
                }
                Ok(Value::Float(x.ln()))
            },
        },
        Builtin {
            name: "log2",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if x <= 0.0 {
                    return Err(runtime_error(expr, "log2 expects positive argument"));
                }
                Ok(Value::Float(x.log2()))
            },
        },
        Builtin {
            name: "log10",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                if x <= 0.0 {
                    return Err(runtime_error(expr, "log10 expects positive argument"));
                }
                Ok(Value::Float(x.log10()))
            },
        },
        Builtin {
            name: "cbrt",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.cbrt()))
            },
        },
        Builtin {
            name: "hypot",
            params: &[ParamType::Float, ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(x), Value::Float(y)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                Ok(Value::Float(x.hypot(*y)))
            },
        },
        Builtin {
            name: "gcd",
            params: &[ParamType::Int, ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let (Value::Int(a), Value::Int(b)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                let mut x = a.abs();
                let mut y = b.abs();
                while y != 0 {
                    let temp = y;
                    y = x % y;
                    x = temp;
                }
                Ok(Value::Int(x))
            },
        },
        Builtin {
            name: "lcm",
            params: &[ParamType::Int, ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let (Value::Int(a), Value::Int(b)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                if *a == 0 || *b == 0 {
                    return Ok(Value::Int(0));
                }
                let mut x = a.abs();
                let mut y = b.abs();
                while y != 0 {
                    let temp = y;
                    y = x % y;
                    x = temp;
                }
                let gcd = x;
                let quotient = a.abs() / gcd;
                let result = quotient
                    .checked_mul(b.abs())
                    .ok_or_else(|| runtime_error(expr, "integer overflow in lcm"))?;
                Ok(Value::Int(result))
            },
        },
        Builtin {
            name: "factorial",
            params: &[ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                if !(0..=20).contains(&n) {
                    return Err(runtime_error(
                        expr,
                        "factorial expects an integer between 0 and 20",
                    ));
                }
                let mut res: i64 = 1;
                for i in 2..=n {
                    res = res
                        .checked_mul(i)
                        .ok_or_else(|| runtime_error(expr, "integer overflow in factorial"))?;
                }
                Ok(Value::Int(res))
            },
        },
        Builtin {
            name: "is_even",
            params: &[ParamType::Int],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(n % 2 == 0))
            },
        },
        Builtin {
            name: "is_odd",
            params: &[ParamType::Int],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(n % 2 != 0))
            },
        },
        Builtin {
            name: "is_prime",
            params: &[ParamType::Int],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Int(n) = values[0] else {
                    unreachable!()
                };
                if n <= 1 {
                    return Ok(Value::Bool(false));
                }
                if n <= 3 {
                    return Ok(Value::Bool(true));
                }
                if n % 2 == 0 || n % 3 == 0 {
                    return Ok(Value::Bool(false));
                }
                let mut i = 5;
                while i * i <= n {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return Ok(Value::Bool(false));
                    }
                    i += 6;
                }
                Ok(Value::Bool(true))
            },
        },
        Builtin {
            name: "next_prime",
            params: &[ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, expr| {
                let Value::Int(mut n) = values[0] else {
                    unreachable!()
                };
                if n < 2 {
                    return Ok(Value::Int(2));
                }
                loop {
                    n = n
                        .checked_add(1)
                        .ok_or_else(|| runtime_error(expr, "integer overflow in next_prime"))?;
                    if is_prime_helper(n) {
                        return Ok(Value::Int(n));
                    }
                }
            },
        },
        Builtin {
            name: "is_nan",
            params: &[ParamType::Float],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(x.is_nan()))
            },
        },
        Builtin {
            name: "is_finite",
            params: &[ParamType::Float],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(x.is_finite()))
            },
        },
        Builtin {
            name: "is_infinite",
            params: &[ParamType::Float],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Bool(x.is_infinite()))
            },
        },
        Builtin {
            name: "sign",
            params: &[ParamType::Int],
            return_type: ReturnType::Int,
            handler: |values, _expr| {
                let Value::Int(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Int(x.signum()))
            },
        },
        Builtin {
            name: "sign_float",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(if x == 0.0 { 0.0 } else { x.signum() }))
            },
        },
        Builtin {
            name: "trunc",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.trunc()))
            },
        },
        Builtin {
            name: "fract",
            params: &[ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let Value::Float(x) = values[0] else {
                    unreachable!()
                };
                Ok(Value::Float(x.fract()))
            },
        },
        Builtin {
            name: "copysign",
            params: &[ParamType::Float, ParamType::Float],
            return_type: ReturnType::Float,
            handler: |values, _expr| {
                let (Value::Float(x), Value::Float(y)) = (&values[0], &values[1]) else {
                    unreachable!()
                };
                Ok(Value::Float(x.copysign(*y)))
            },
        },
        Builtin {
            name: "is_between",
            params: &[ParamType::Int, ParamType::Int, ParamType::Int],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let (Value::Int(v), Value::Int(low), Value::Int(high)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                Ok(Value::Bool(*v >= *low && *v <= *high))
            },
        },
        Builtin {
            name: "is_between_float",
            params: &[ParamType::Float, ParamType::Float, ParamType::Float],
            return_type: ReturnType::Bool,
            handler: |values, _expr| {
                let (Value::Float(v), Value::Float(low), Value::Float(high)) =
                    (&values[0], &values[1], &values[2])
                else {
                    unreachable!()
                };
                Ok(Value::Bool(*v >= *low && *v <= *high))
            },
        },
    ]
}

fn is_prime_helper(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
