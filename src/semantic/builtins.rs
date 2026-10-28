use super::{
    Analyzer,
    symbol::{error_expr, numeric_type, require_arity},
};
use crate::{
    ast::{Expr, Type},
    diagnostic::Diagnostic,
};

pub(super) const RESERVED_BUILTINS: &[&str] = &[
    "len",
    "to_string",
    "to_int",
    "to_float",
    "range",
    "append",
    "read_line",
    "input",
    "upper",
    "lower",
    "trim",
    "trim_start",
    "trim_end",
    "contains",
    "starts_with",
    "ends_with",
    "replace",
    "split",
    "join",
    "repeat",
    "find",
    "count",
    "slice",
    "abs",
    "min",
    "max",
    "clamp",
    "pow",
    "sqrt",
    "round",
    "floor",
    "ceil",
    "reverse",
    "sort",
    "sum",
    "mean",
    "median",
    "variance",
    "stdev",
    "insert",
    "remove",
    "contains_key",
    "keys",
    "values",
    "remove_key",
];

pub(super) fn is_reserved(name: &str) -> bool {
    RESERVED_BUILTINS.contains(&name)
}

impl Analyzer {
    pub(super) fn call_type(
        &mut self,
        callee: &str,
        arguments: &[Expr],
        expression: &Expr,
    ) -> Result<Type, Diagnostic> {
        if let Some(signature) = self.functions.get(callee).cloned() {
            if signature.parameters.len() != arguments.len() {
                return Err(error_expr(
                    expression,
                    format!(
                        "function '{callee}' expects {} arguments, found {}",
                        signature.parameters.len(),
                        arguments.len()
                    ),
                ));
            }
            for (argument, expected) in arguments.iter().zip(&signature.parameters) {
                let actual = self.expression_type_as(argument, expected)?;
                self.require_type(expected, &actual, argument)?;
            }
            return Ok(signature.return_type);
        }

        if let Some(class_name) = &self.current_class
            && let Some(class_info) = self.classes.get(class_name)
            && let Some(signature) = class_info.methods.get(callee).cloned()
        {
            if signature.parameters.len() != arguments.len() {
                return Err(error_expr(
                    expression,
                    format!(
                        "method '{callee}' expects {} argument{}, found {}",
                        signature.parameters.len(),
                        if signature.parameters.len() == 1 {
                            ""
                        } else {
                            "s"
                        },
                        arguments.len()
                    ),
                ));
            }
            for (argument, expected) in arguments.iter().zip(&signature.parameters) {
                let actual = self.expression_type_as(argument, expected)?;
                self.require_type(expected, &actual, argument)?;
            }
            return Ok(signature.return_type);
        }

        if let Some(builtin) = crate::builtins::find_builtin(callee) {
            return self.check_registry_builtin(builtin, arguments, expression);
        }

        match callee {
            "read_line" | "input" => {
                if arguments.len() > 1 {
                    return Err(error_expr(
                        expression,
                        format!(
                            "{callee} expects 0 or 1 arguments, found {}",
                            arguments.len()
                        ),
                    ));
                }
                if let Some(prompt) = arguments.first() {
                    self.expect_expression(prompt, &Type::String)?;
                }
                Ok(Type::String)
            }
            "len" => {
                require_arity(callee, arguments, 1, expression)?;
                match self.expression_type(&arguments[0])? {
                    Type::String | Type::Array(_) | Type::Map { .. } | Type::Any => Ok(Type::Int),
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("len expects a string, array, or map, found {actual}"),
                    )),
                }
            }
            "to_string" => {
                require_arity(callee, arguments, 1, expression)?;
                let actual = self.expression_type(&arguments[0])?;
                if actual == Type::Void {
                    Err(error_expr(&arguments[0], "cannot convert void to string"))
                } else {
                    Ok(Type::String)
                }
            }
            "to_int" => {
                require_arity(callee, arguments, 1, expression)?;
                let actual = self.expression_type(&arguments[0])?;
                if matches!(actual, Type::Int | Type::Float | Type::String) {
                    Ok(Type::Int)
                } else {
                    Err(error_expr(
                        &arguments[0],
                        format!("to_int expects an int, float or string, found {actual}"),
                    ))
                }
            }
            "to_float" => {
                require_arity(callee, arguments, 1, expression)?;
                let actual = self.expression_type(&arguments[0])?;
                if matches!(actual, Type::Int | Type::Float | Type::String) {
                    Ok(Type::Float)
                } else {
                    Err(error_expr(
                        &arguments[0],
                        format!("to_float expects an int, float or string, found {actual}"),
                    ))
                }
            }
            "upper" | "lower" | "trim" | "trim_start" | "trim_end" => {
                require_arity(callee, arguments, 1, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                Ok(Type::String)
            }
            "contains" => {
                require_arity(callee, arguments, 2, expression)?;
                match self.expression_type(&arguments[0])? {
                    Type::String => {
                        self.expect_expression(&arguments[1], &Type::String)?;
                        Ok(Type::Bool)
                    }
                    Type::Array(element) => {
                        let actual = self.expression_type(&arguments[1])?;
                        self.require_type(&element, &actual, &arguments[1])?;
                        Ok(Type::Bool)
                    }
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("contains expects a string or array, found {actual}"),
                    )),
                }
            }
            "starts_with" | "ends_with" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::String)?;
                Ok(Type::Bool)
            }
            "replace" => {
                require_arity(callee, arguments, 3, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::String)?;
                self.expect_expression(&arguments[2], &Type::String)?;
                Ok(Type::String)
            }
            "split" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::String)?;
                Ok(Type::array(Type::String))
            }
            "join" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::array(Type::String))?;
                Ok(Type::String)
            }
            "repeat" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::Int)?;
                Ok(Type::String)
            }
            "find" | "count" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::String)?;
                self.expect_expression(&arguments[1], &Type::String)?;
                Ok(Type::Int)
            }
            "slice" => {
                if !(2..=3).contains(&arguments.len()) {
                    return Err(error_expr(
                        expression,
                        format!(
                            "{callee} expects 2 or 3 arguments, found {}",
                            arguments.len()
                        ),
                    ));
                }
                let result = match self.expression_type(&arguments[0])? {
                    Type::String => Type::String,
                    Type::Array(element) => Type::Array(element),
                    actual => {
                        return Err(error_expr(
                            &arguments[0],
                            format!("slice expects a string or array, found {actual}"),
                        ));
                    }
                };
                self.expect_expression(&arguments[1], &Type::Int)?;
                if let Some(end) = arguments.get(2) {
                    self.expect_expression(end, &Type::Int)?;
                }
                Ok(result)
            }
            "abs" => {
                require_arity(callee, arguments, 1, expression)?;
                numeric_type(self.expression_type(&arguments[0])?, &arguments[0], callee)
            }
            "min" | "max" => {
                if arguments.len() == 1 {
                    let Type::Array(element) = self.expression_type(&arguments[0])? else {
                        return Err(error_expr(
                            &arguments[0],
                            format!("{callee} expects a number or array"),
                        ));
                    };
                    if !matches!(element.as_ref(), Type::Int | Type::Float | Type::String) {
                        return Err(error_expr(
                            &arguments[0],
                            format!("unsupported {callee} element type"),
                        ));
                    }
                    Ok(*element)
                } else if arguments.len() == 2 {
                    let value_type =
                        numeric_type(self.expression_type(&arguments[0])?, &arguments[0], callee)?;
                    self.expect_expression(&arguments[1], &value_type)?;
                    Ok(value_type)
                } else {
                    Err(error_expr(
                        expression,
                        format!(
                            "{callee} expects 1 or 2 arguments, found {}",
                            arguments.len()
                        ),
                    ))
                }
            }
            "mean" | "median" | "variance" | "stdev" => {
                require_arity(callee, arguments, 1, expression)?;
                let Type::Array(element) = self.expression_type(&arguments[0])? else {
                    return Err(error_expr(
                        &arguments[0],
                        format!("{callee} expects an array"),
                    ));
                };
                if !matches!(element.as_ref(), Type::Int | Type::Float) {
                    return Err(error_expr(
                        &arguments[0],
                        format!("{callee} expects a numeric array"),
                    ));
                }
                Ok(Type::Float)
            }
            "clamp" => {
                require_arity(callee, arguments, 3, expression)?;
                let value_type =
                    numeric_type(self.expression_type(&arguments[0])?, &arguments[0], callee)?;
                self.expect_expression(&arguments[1], &value_type)?;
                self.expect_expression(&arguments[2], &value_type)?;
                Ok(value_type)
            }
            "pow" => {
                require_arity(callee, arguments, 2, expression)?;
                self.expect_expression(&arguments[0], &Type::Float)?;
                self.expect_expression(&arguments[1], &Type::Float)?;
                Ok(Type::Float)
            }
            "sqrt" => {
                require_arity(callee, arguments, 1, expression)?;
                self.expect_expression(&arguments[0], &Type::Float)?;
                Ok(Type::Float)
            }
            "round" | "floor" | "ceil" => {
                require_arity(callee, arguments, 1, expression)?;
                self.expect_expression(&arguments[0], &Type::Float)?;
                Ok(Type::Int)
            }
            "reverse" => {
                require_arity(callee, arguments, 1, expression)?;
                let value_type = self.expression_type(&arguments[0])?;
                if matches!(value_type, Type::String | Type::Array(_)) {
                    Ok(value_type)
                } else {
                    Err(error_expr(
                        &arguments[0],
                        "reverse expects a string or array",
                    ))
                }
            }
            "sort" | "sum" => {
                require_arity(callee, arguments, 1, expression)?;
                let Type::Array(element) = self.expression_type(&arguments[0])? else {
                    return Err(error_expr(
                        &arguments[0],
                        format!("{callee} expects an array"),
                    ));
                };
                if !matches!(element.as_ref(), Type::Int | Type::Float | Type::String)
                    || (callee == "sum" && element.as_ref() == &Type::String)
                {
                    return Err(error_expr(
                        &arguments[0],
                        format!("unsupported {callee} element type"),
                    ));
                }
                if callee == "sum" {
                    Ok(*element)
                } else {
                    Ok(Type::Array(element))
                }
            }
            "insert" => {
                require_arity(callee, arguments, 3, expression)?;
                let array_type = self.expression_type(&arguments[0])?;
                let Type::Array(element) = &array_type else {
                    return Err(error_expr(&arguments[0], "insert expects an array"));
                };
                self.expect_expression(&arguments[1], &Type::Int)?;
                self.expect_expression(&arguments[2], element)?;
                Ok(array_type)
            }
            "remove" => {
                require_arity(callee, arguments, 2, expression)?;
                let array_type = self.expression_type(&arguments[0])?;
                if !matches!(array_type, Type::Array(_)) {
                    return Err(error_expr(&arguments[0], "remove expects an array"));
                }
                self.expect_expression(&arguments[1], &Type::Int)?;
                Ok(array_type)
            }
            "range" => {
                if !(1..=3).contains(&arguments.len()) {
                    return Err(error_expr(
                        expression,
                        format!("range expects 1 to 3 arguments, found {}", arguments.len()),
                    ));
                }
                for argument in arguments {
                    self.expect_expression(argument, &Type::Int)?;
                }
                Ok(Type::array(Type::Int))
            }
            "append" => {
                require_arity(callee, arguments, 2, expression)?;
                let array_type = self.expression_type(&arguments[0])?;
                let Type::Array(element) = &array_type else {
                    return Err(error_expr(
                        &arguments[0],
                        "append expects an array as its first argument",
                    ));
                };
                let actual = self.expression_type(&arguments[1])?;
                self.require_type(element, &actual, &arguments[1])?;
                Ok(array_type)
            }
            "contains_key" => {
                require_arity(callee, arguments, 2, expression)?;
                let map_type = self.expression_type(&arguments[0])?;
                match map_type {
                    Type::Map { key, .. } => {
                        let actual = self.expression_type(&arguments[1])?;
                        self.require_type(&key, &actual, &arguments[1])?;
                        Ok(Type::Bool)
                    }
                    Type::Any => Ok(Type::Bool),
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("contains_key expects a map, found {actual}"),
                    )),
                }
            }
            "keys" => {
                require_arity(callee, arguments, 1, expression)?;
                match self.expression_type(&arguments[0])? {
                    Type::Map { key, .. } => Ok(Type::array(*key)),
                    Type::Any => Ok(Type::array(Type::Any)),
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("keys expects a map, found {actual}"),
                    )),
                }
            }
            "values" => {
                require_arity(callee, arguments, 1, expression)?;
                match self.expression_type(&arguments[0])? {
                    Type::Map { value, .. } => Ok(Type::array(*value)),
                    Type::Any => Ok(Type::array(Type::Any)),
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("values expects a map, found {actual}"),
                    )),
                }
            }
            "remove_key" => {
                require_arity(callee, arguments, 2, expression)?;
                let map_type = self.expression_type(&arguments[0])?;
                match map_type {
                    Type::Map { key, value } => {
                        let actual = self.expression_type(&arguments[1])?;
                        self.require_type(&key, &actual, &arguments[1])?;
                        Ok(Type::map(*key, *value))
                    }
                    Type::Any => Ok(Type::dict()),
                    actual => Err(error_expr(
                        &arguments[0],
                        format!("remove_key expects a map, found {actual}"),
                    )),
                }
            }
            _ => Err(error_expr(
                expression,
                format!("undefined function '{callee}'"),
            )),
        }
    }

    fn check_registry_builtin(
        &mut self,
        builtin: &crate::builtins::Builtin,
        arguments: &[Expr],
        expression: &Expr,
    ) -> Result<Type, Diagnostic> {
        if arguments.len() != builtin.params.len() {
            return Err(error_expr(
                expression,
                format!(
                    "{} expects {} argument{}, found {}",
                    builtin.name,
                    builtin.params.len(),
                    if builtin.params.len() == 1 { "" } else { "s" },
                    arguments.len()
                ),
            ));
        }
        let mut arg_types = Vec::with_capacity(arguments.len());
        for (arg, param) in arguments.iter().zip(builtin.params) {
            let actual = self.expression_type(arg)?;
            match param {
                crate::builtins::ParamType::Int => {
                    self.require_type(&Type::Int, &actual, arg)?;
                }
                crate::builtins::ParamType::Float => {
                    self.require_type(&Type::Float, &actual, arg)?;
                }
                crate::builtins::ParamType::Bool => {
                    self.require_type(&Type::Bool, &actual, arg)?;
                }
                crate::builtins::ParamType::String => {
                    self.require_type(&Type::String, &actual, arg)?;
                }
                crate::builtins::ParamType::IntArray => {
                    self.require_type(&Type::array(Type::Int), &actual, arg)?;
                }
                crate::builtins::ParamType::FloatArray => {
                    self.require_type(&Type::array(Type::Float), &actual, arg)?;
                }
                crate::builtins::ParamType::StringArray => {
                    self.require_type(&Type::array(Type::String), &actual, arg)?;
                }
                crate::builtins::ParamType::BoolArray => {
                    self.require_type(&Type::array(Type::Bool), &actual, arg)?;
                }
                crate::builtins::ParamType::Int2DArray => {
                    self.require_type(&Type::array(Type::array(Type::Int)), &actual, arg)?;
                }
                crate::builtins::ParamType::Float2DArray => {
                    self.require_type(&Type::array(Type::array(Type::Float)), &actual, arg)?;
                }
                crate::builtins::ParamType::String2DArray => {
                    self.require_type(&Type::array(Type::array(Type::String)), &actual, arg)?;
                }
                crate::builtins::ParamType::AnyArray => {
                    if !matches!(actual, Type::Array(_)) {
                        return Err(error_expr(
                            arg,
                            format!("{} expects an array, found {actual}", builtin.name),
                        ));
                    }
                }
                crate::builtins::ParamType::NumericArray => {
                    if !matches!(&actual, Type::Array(elem) if matches!(elem.as_ref(), Type::Int | Type::Float))
                    {
                        return Err(error_expr(
                            arg,
                            format!("{} expects a numeric array, found {actual}", builtin.name),
                        ));
                    }
                }
            }
            arg_types.push(actual);
        }
        match &builtin.return_type {
            crate::builtins::ReturnType::Int => Ok(Type::Int),
            crate::builtins::ReturnType::Float => Ok(Type::Float),
            crate::builtins::ReturnType::Bool => Ok(Type::Bool),
            crate::builtins::ReturnType::String => Ok(Type::String),
            crate::builtins::ReturnType::IntArray => Ok(Type::array(Type::Int)),
            crate::builtins::ReturnType::FloatArray => Ok(Type::array(Type::Float)),
            crate::builtins::ReturnType::StringArray => Ok(Type::array(Type::String)),
            crate::builtins::ReturnType::BoolArray => Ok(Type::array(Type::Bool)),
            crate::builtins::ReturnType::Int2DArray => Ok(Type::array(Type::array(Type::Int))),
            crate::builtins::ReturnType::Float2DArray => Ok(Type::array(Type::array(Type::Float))),
            crate::builtins::ReturnType::String2DArray => {
                Ok(Type::array(Type::array(Type::String)))
            }
            crate::builtins::ReturnType::SameAsArg(idx) => Ok(arg_types[*idx].clone()),
            crate::builtins::ReturnType::ElementOfArg(idx) => {
                let Type::Array(elem) = &arg_types[*idx] else {
                    unreachable!()
                };
                Ok((**elem).clone())
            }
            crate::builtins::ReturnType::ArrayOfArg(idx) => {
                Ok(Type::array(arg_types[*idx].clone()))
            }
        }
    }
}
