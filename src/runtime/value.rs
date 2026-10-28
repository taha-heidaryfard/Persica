use crate::ast::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Void,
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Bool(_) => Type::Bool,
            Self::String(_) => Type::String,
            Self::Array(values) => {
                Type::array(values.first().map_or(Type::Void, Value::value_type))
            }
            Self::Map(entries) => {
                let (key_type, val_type) = if let Some((first_k, first_v)) = entries.first() {
                    let k_type = first_k.value_type();
                    let v_type = first_v.value_type();
                    let uniform_k = entries.iter().all(|(k, _)| k.value_type() == k_type);
                    let uniform_v = entries.iter().all(|(_, v)| v.value_type() == v_type);
                    (
                        if uniform_k { k_type } else { Type::Any },
                        if uniform_v { v_type } else { Type::Any },
                    )
                } else {
                    (Type::Any, Type::Any)
                };
                Type::map(key_type, val_type)
            }
            Self::Void => Type::Void,
        }
    }

    pub fn is_assignable_to(&self, expected: &Type) -> bool {
        if expected == &Type::Any {
            return true;
        }
        match (expected, self) {
            (Type::Int, Value::Int(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::String, Value::String(_)) => true,
            (Type::Array(expected_elem), Value::Array(values)) => {
                if values.is_empty() {
                    true
                } else {
                    values.iter().all(|v| v.is_assignable_to(expected_elem))
                }
            }
            (
                Type::Map {
                    key: exp_k,
                    value: exp_v,
                },
                Value::Map(entries),
            ) => {
                if entries.is_empty() {
                    true
                } else {
                    entries
                        .iter()
                        .all(|(k, v)| k.is_assignable_to(exp_k) && v.is_assignable_to(exp_v))
                }
            }
            _ => false,
        }
    }

    pub(crate) fn nested(&self) -> String {
        match self {
            Self::String(value) => format!("\"{value}\""),
            _ => self.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) if value.fract() == 0.0 => write!(f, "{value:.1}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::String(value) => f.write_str(value),
            Self::Array(values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(Value::nested)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Map(entries) => write!(
                f,
                "{{{}}}",
                entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.nested(), v.nested()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Void => f.write_str("void"),
        }
    }
}
