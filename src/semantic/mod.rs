mod builtins;
mod expr;
mod stmt;
mod symbol;

use crate::{
    ast::{Program, StmtKind, Type},
    diagnostic::Diagnostic,
};
use std::collections::HashMap;
use symbol::{ClassInfo, FunctionSignature, StructInfo, VariableSymbol, error};

pub fn validate(program: &Program) -> Result<(), Diagnostic> {
    Analyzer::new().validate(program)
}

struct Analyzer {
    scopes: Vec<HashMap<String, VariableSymbol>>,
    functions: HashMap<String, FunctionSignature>,
    structs: HashMap<String, StructInfo>,
    classes: HashMap<String, ClassInfo>,
    current_class: Option<String>,
    loop_depth: usize,
    current_return: Option<Type>,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            structs: HashMap::new(),
            classes: HashMap::new(),
            current_class: None,
            loop_depth: 0,
            current_return: None,
        }
    }

    fn validate(mut self, program: &Program) -> Result<(), Diagnostic> {
        self.collect_definitions(program)?;
        for statement in program {
            self.statement(statement)?;
        }
        Ok(())
    }

    fn collect_definitions(&mut self, program: &Program) -> Result<(), Diagnostic> {
        for statement in program {
            match &statement.kind {
                StmtKind::StructDeclaration { name, fields } => {
                    if builtins::is_reserved(name.as_str()) {
                        return Err(error(
                            statement,
                            format!("'{name}' is a reserved built-in function"),
                        ));
                    }
                    if self.structs.contains_key(name)
                        || self.classes.contains_key(name)
                        || self.functions.contains_key(name)
                    {
                        return Err(error(
                            statement,
                            format!("type '{name}' is already defined"),
                        ));
                    }
                    let mut field_names = HashMap::new();
                    let mut field_list = Vec::new();
                    for field in fields {
                        if field.field_type == Type::Void {
                            return Err(error(
                                statement,
                                format!("field '{}' cannot have void type", field.name),
                            ));
                        }
                        if field_names.insert(field.name.clone(), ()).is_some() {
                            return Err(error(
                                statement,
                                format!("duplicate field '{}' in struct '{name}'", field.name),
                            ));
                        }
                        field_list.push((field.name.clone(), field.field_type.clone()));
                    }
                    self.structs.insert(
                        name.clone(),
                        StructInfo {
                            fields: field_list.clone(),
                        },
                    );
                    self.functions.insert(
                        name.clone(),
                        FunctionSignature {
                            parameters: field_list.into_iter().map(|(_, ty)| ty).collect(),
                            return_type: Type::Custom(name.clone()),
                        },
                    );
                }
                StmtKind::ClassDeclaration {
                    name,
                    fields,
                    methods,
                } => {
                    if builtins::is_reserved(name.as_str()) {
                        return Err(error(
                            statement,
                            format!("'{name}' is a reserved built-in function"),
                        ));
                    }
                    if self.structs.contains_key(name)
                        || self.classes.contains_key(name)
                        || self.functions.contains_key(name)
                    {
                        return Err(error(
                            statement,
                            format!("type '{name}' is already defined"),
                        ));
                    }
                    let mut field_names = HashMap::new();
                    let mut field_list = Vec::new();
                    for field in fields {
                        if field.field_type == Type::Void {
                            return Err(error(
                                statement,
                                format!("field '{}' cannot have void type", field.name),
                            ));
                        }
                        if field_names.insert(field.name.clone(), ()).is_some() {
                            return Err(error(
                                statement,
                                format!("duplicate field '{}' in class '{name}'", field.name),
                            ));
                        }
                        field_list.push((field.name.clone(), field.field_type.clone()));
                    }
                    let mut method_map = HashMap::new();
                    for method in methods {
                        if field_names.contains_key(&method.name) {
                            return Err(error(
                                statement,
                                format!(
                                    "member '{}' is already defined as a field in class '{name}'",
                                    method.name
                                ),
                            ));
                        }
                        if method_map.contains_key(&method.name) {
                            return Err(error(
                                statement,
                                format!("duplicate method '{}' in class '{name}'", method.name),
                            ));
                        }
                        method_map.insert(
                            method.name.clone(),
                            FunctionSignature {
                                parameters: method
                                    .parameters
                                    .iter()
                                    .map(|p| p.parameter_type.clone())
                                    .collect(),
                                return_type: method.return_type.clone(),
                            },
                        );
                    }
                    self.classes.insert(
                        name.clone(),
                        ClassInfo {
                            fields: field_list.clone(),
                            methods: method_map,
                        },
                    );
                    self.functions.insert(
                        name.clone(),
                        FunctionSignature {
                            parameters: field_list.into_iter().map(|(_, ty)| ty).collect(),
                            return_type: Type::Custom(name.clone()),
                        },
                    );
                }
                StmtKind::FunctionDeclaration {
                    return_type,
                    name,
                    parameters,
                    ..
                } => {
                    if builtins::is_reserved(name.as_str()) {
                        return Err(error(
                            statement,
                            format!("'{name}' is a reserved built-in function"),
                        ));
                    }
                    if self.functions.contains_key(name)
                        || self.structs.contains_key(name)
                        || self.classes.contains_key(name)
                    {
                        return Err(error(
                            statement,
                            format!("function '{name}' is already defined"),
                        ));
                    }
                    self.functions.insert(
                        name.clone(),
                        FunctionSignature {
                            parameters: parameters
                                .iter()
                                .map(|parameter| parameter.parameter_type.clone())
                                .collect(),
                            return_type: return_type.clone(),
                        },
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn in_loop<T>(
        &mut self,
        action: impl FnOnce(&mut Self) -> Result<T, Diagnostic>,
    ) -> Result<T, Diagnostic> {
        self.loop_depth += 1;
        let result = action(self);
        self.loop_depth -= 1;
        result
    }
}
