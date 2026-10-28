use crate::ast::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
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
            Self::Void => Type::Void,
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
            Self::Void => f.write_str("void"),
        }
    }
}
