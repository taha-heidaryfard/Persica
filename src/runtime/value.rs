use super::ModuleInstance;
use crate::ast::Type;
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
    pub name: String,
    pub fields: Vec<(String, Value)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Struct {
        name: String,
        fields: Vec<(String, Value)>,
    },
    Class(Rc<RefCell<ClassInstance>>),
    Module(Rc<RefCell<ModuleInstance>>),
    Void,
}

impl Value {
    pub fn default_for_type(ty: &Type) -> Self {
        match ty {
            Type::Int => Self::Int(0),
            Type::Float => Self::Float(0.0),
            Type::Bool => Self::Bool(false),
            Type::String => Self::String(String::new()),
            Type::Array(_) => Self::Array(Vec::new()),
            Type::Map { .. } => Self::Map(Vec::new()),
            _ => Self::Void,
        }
    }

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
            Self::Struct { name, .. } => Type::Custom(name.clone()),
            Self::Class(inst) => Type::Custom(inst.borrow().name.clone()),
            Self::Module(_) => Type::Module,
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
            (Type::Module, Value::Module(_)) => true,
            (Type::Custom(exp_name), Value::Struct { name, .. }) => exp_name == name,
            (Type::Custom(exp_name), Value::Class(inst)) => exp_name == &inst.borrow().name,
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
            Self::Struct { name, fields } => {
                if fields.is_empty() {
                    write!(f, "{name} {{}}")
                } else {
                    write!(
                        f,
                        "{name} {{ {} }}",
                        fields
                            .iter()
                            .map(|(k, v)| format!("{k}: {}", v.nested()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Self::Class(inst) => {
                let inst = inst.borrow();
                if inst.fields.is_empty() {
                    write!(f, "{} {{}}", inst.name)
                } else {
                    write!(
                        f,
                        "{} {{ {} }}",
                        inst.name,
                        inst.fields
                            .iter()
                            .map(|(k, v)| format!("{k}: {}", v.nested()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Self::Module(m) => write!(f, "<module '{}'>", m.borrow().name),
            Self::Void => f.write_str("void"),
        }
    }
}
