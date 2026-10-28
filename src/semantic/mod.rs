mod builtins;
mod expr;
mod stmt;
mod symbol;

use crate::{
    ast::{Program, StmtKind, Type},
    diagnostic::{Diagnostic, Phase},
};
use std::collections::{HashMap, HashSet};
use symbol::{ClassInfo, FunctionSignature, ModuleSymbol, StructInfo, VariableSymbol, error};

pub fn validate(program: &Program) -> Result<(), Diagnostic> {
    let mut analyzer = Analyzer::new();
    analyzer.load_prelude()?;
    analyzer.validate(program)
}

struct Analyzer {
    scopes: Vec<HashMap<String, VariableSymbol>>,
    functions: HashMap<String, FunctionSignature>,
    structs: HashMap<String, StructInfo>,
    classes: HashMap<String, ClassInfo>,
    modules: HashMap<String, ModuleSymbol>,
    loaded_module_exports: HashMap<String, ModuleSymbol>,
    type_aliases: HashMap<String, String>,
    declared_names: HashSet<String>,
    loading_stack: Vec<String>,
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
            modules: HashMap::new(),
            loaded_module_exports: HashMap::new(),
            type_aliases: HashMap::new(),
            declared_names: HashSet::new(),
            loading_stack: Vec::new(),
            current_class: None,
            loop_depth: 0,
            current_return: None,
        }
    }

    pub fn load_prelude(&mut self) -> Result<(), Diagnostic> {
        let prelude = crate::stdlib::get_prelude_program();
        self.collect_definitions(prelude)?;
        for statement in prelude {
            self.statement(statement)?;
        }
        self.declared_names.clear();
        Ok(())
    }

    pub(super) fn canonical_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Custom(name) => {
                let mut current = name.clone();
                while let Some(target) = self.type_aliases.get(&current) {
                    current = target.clone();
                }
                Type::Custom(current)
            }
            Type::Array(inner) => Type::array(self.canonical_type(inner)),
            Type::Map { key, value } => {
                Type::map(self.canonical_type(key), self.canonical_type(value))
            }
            other => other.clone(),
        }
    }

    fn validate(&mut self, program: &Program) -> Result<(), Diagnostic> {
        self.collect_definitions(program)?;
        for statement in program {
            self.statement(statement)?;
        }
        Ok(())
    }

    fn collect_definitions(&mut self, program: &Program) -> Result<(), Diagnostic> {
        for statement in program {
            match &statement.kind {
                StmtKind::Import { module, items } => {
                    self.process_import(module, items, statement)?;
                }
                StmtKind::StructDeclaration { name, fields } => {
                    if builtins::is_reserved(name.as_str()) {
                        return Err(error(
                            statement,
                            format!("'{name}' is a reserved built-in function"),
                        ));
                    }
                    if self.declared_names.contains(name) {
                        return Err(error(
                            statement,
                            format!("type '{name}' is already defined"),
                        ));
                    }
                    self.declared_names.insert(name.clone());
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
                    if self.declared_names.contains(name) {
                        return Err(error(
                            statement,
                            format!("type '{name}' is already defined"),
                        ));
                    }
                    self.declared_names.insert(name.clone());
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
                    if self.declared_names.contains(name) {
                        return Err(error(
                            statement,
                            format!("function '{name}' is already defined"),
                        ));
                    }
                    self.declared_names.insert(name.clone());
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

    fn process_import(
        &mut self,
        module_specifier: &str,
        items: &crate::ast::ImportItems,
        statement: &crate::ast::Stmt,
    ) -> Result<(), Diagnostic> {
        if self.loading_stack.iter().any(|m| m == module_specifier) {
            return Err(error(
                statement,
                format!("circular import detected: '{module_specifier}'"),
            ));
        }

        let module_symbol =
            if let Some(sym) = self.loaded_module_exports.get(module_specifier).cloned() {
                sym
            } else {
                let (key, source) =
                    crate::module::load_module_source(module_specifier, None, statement.span)?;
                let module_ast = crate::module::parse_module(&source, statement.span)?;
                self.loading_stack.push(module_specifier.to_string());
                let mut sub_analyzer = Analyzer::new();
                sub_analyzer.loaded_module_exports = self.loaded_module_exports.clone();
                sub_analyzer.loading_stack = self.loading_stack.clone();
                sub_analyzer.validate(&module_ast)?;
                self.loading_stack.pop();
                self.loaded_module_exports = sub_analyzer.loaded_module_exports;
                let mod_sym = ModuleSymbol {
                    name: module_specifier.to_string(),
                    functions: sub_analyzer.functions,
                    structs: sub_analyzer.structs,
                    classes: sub_analyzer.classes,
                    variables: sub_analyzer.scopes.into_iter().next().unwrap_or_default(),
                };
                self.loaded_module_exports
                    .insert(module_specifier.to_string(), mod_sym.clone());
                self.loaded_module_exports.insert(key, mod_sym.clone());
                mod_sym
            };

        match items {
            crate::ast::ImportItems::Module { alias } => {
                let default_alias = module_specifier
                    .split('.')
                    .next_back()
                    .unwrap_or(module_specifier);
                let effective_alias = alias.as_deref().unwrap_or(default_alias);
                self.modules
                    .insert(effective_alias.to_string(), module_symbol);
                self.scopes.last_mut().unwrap().insert(
                    effective_alias.to_string(),
                    VariableSymbol {
                        variable_type: Type::Module,
                        is_final: true,
                    },
                );
            }
            crate::ast::ImportItems::All => {
                for (name, sig) in module_symbol.functions {
                    self.functions.insert(name, sig);
                }
                for (name, info) in module_symbol.structs {
                    self.structs.insert(name, info);
                }
                for (name, info) in module_symbol.classes {
                    self.classes.insert(name, info);
                }
                for (name, var) in module_symbol.variables {
                    self.scopes.last_mut().unwrap().insert(name, var);
                }
            }
            crate::ast::ImportItems::Named(item_list) => {
                for item in item_list {
                    let target_name = item.alias.as_ref().unwrap_or(&item.name);
                    let mut found = false;
                    if let Some(c_info) = module_symbol.classes.get(&item.name).cloned() {
                        self.classes.insert(target_name.clone(), c_info);
                        if let Some(constructor_sig) =
                            module_symbol.functions.get(&item.name).cloned()
                        {
                            self.functions.insert(target_name.clone(), constructor_sig);
                        }
                        if target_name != &item.name {
                            self.type_aliases
                                .insert(target_name.clone(), item.name.clone());
                        }
                        found = true;
                    }
                    if let Some(s_info) = module_symbol.structs.get(&item.name).cloned() {
                        self.structs.insert(target_name.clone(), s_info);
                        if let Some(constructor_sig) =
                            module_symbol.functions.get(&item.name).cloned()
                        {
                            self.functions.insert(target_name.clone(), constructor_sig);
                        }
                        if target_name != &item.name {
                            self.type_aliases
                                .insert(target_name.clone(), item.name.clone());
                        }
                        found = true;
                    }
                    if let Some(fn_sig) = module_symbol.functions.get(&item.name).cloned() {
                        self.functions.insert(target_name.clone(), fn_sig);
                        found = true;
                    }
                    if let Some(var_sym) = module_symbol.variables.get(&item.name).cloned() {
                        self.scopes
                            .last_mut()
                            .unwrap()
                            .insert(target_name.clone(), var_sym);
                        found = true;
                    }
                    if !found {
                        return Err(Diagnostic::new(
                            Phase::Semantic,
                            format!(
                                "cannot import '{}' from module '{}': symbol not found",
                                item.name, module_specifier
                            ),
                            item.span,
                        ));
                    }
                }
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
