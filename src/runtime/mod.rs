mod builtins;
mod eval;
mod value;

pub use value::Value;

use crate::{
    ast::{Expr, FieldDef, MethodDef, Parameter, Program, Stmt, StmtKind, Type},
    diagnostic::{Diagnostic, Phase},
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    io::{self, Write},
    path::PathBuf,
    rc::Rc,
};
use value::ClassInstance;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Binding {
    pub(super) variable_type: Type,
    pub(super) is_final: bool,
    pub(super) value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Function {
    pub(super) return_type: Type,
    pub(super) parameters: Vec<Parameter>,
    pub(super) body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct StructDef {
    pub(super) fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ClassDef {
    pub(super) fields: Vec<FieldDef>,
    pub(super) methods: HashMap<String, MethodDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInstance {
    pub name: String,
    pub(super) functions: HashMap<String, Function>,
    pub(super) structs: HashMap<String, StructDef>,
    pub(super) classes: HashMap<String, ClassDef>,
    pub(super) globals: HashMap<String, Binding>,
}

pub(super) enum Flow {
    Normal,
    Break,
    Continue,
    Return(Value),
}

#[derive(Debug, Default)]
pub(super) enum Input {
    #[default]
    Disabled,
    Lines(VecDeque<String>),
    Stdin,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    pub(super) scopes: Vec<HashMap<String, Binding>>,
    pub(super) functions: HashMap<String, Function>,
    pub(super) structs: HashMap<String, StructDef>,
    pub(super) classes: HashMap<String, ClassDef>,
    pub(super) loaded_modules: HashMap<String, Rc<RefCell<ModuleInstance>>>,
    pub(super) type_aliases: HashMap<String, String>,
    pub(super) current_instance: Option<Rc<RefCell<ClassInstance>>>,
    pub(super) output: String,
    pub(super) print_calls: Vec<String>,
    pub(super) call_depth: usize,
    pub(super) input: Input,
    pub(super) stream_output: bool,
    pub(super) prelude_loaded: bool,
    pub(super) current_file_dir: Option<PathBuf>,
    pub(super) active_exception: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            ..Self::default()
        }
    }

    pub fn set_current_file_dir(&mut self, dir: PathBuf) {
        self.current_file_dir = Some(dir);
    }

    pub fn with_base_dir(mut self, dir: Option<PathBuf>) -> Self {
        self.current_file_dir = dir;
        self
    }

    pub fn load_prelude(&mut self) -> Result<(), Diagnostic> {
        if self.prelude_loaded {
            return Ok(());
        }
        self.prelude_loaded = true;
        let prelude = crate::stdlib::get_prelude_program();
        self.execute_program(prelude)
    }

    pub fn with_stdin() -> Self {
        Self {
            input: Input::Stdin,
            ..Self::new()
        }
    }

    pub fn with_stdio() -> Self {
        Self {
            input: Input::Stdin,
            stream_output: true,
            ..Self::new()
        }
    }

    pub fn with_input<I, S>(lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            input: Input::Lines(lines.into_iter().map(Into::into).collect()),
            ..Self::new()
        }
    }

    pub fn interpret(mut self, program: &Program) -> Result<Vec<String>, Diagnostic> {
        self.load_prelude()?;
        self.execute_program(program)?;
        Ok(self.print_calls)
    }

    pub fn interpret_text(mut self, program: &Program) -> Result<String, Diagnostic> {
        self.load_prelude()?;
        self.execute_program(program)?;
        Ok(self.output)
    }

    fn execute_program(&mut self, program: &Program) -> Result<(), Diagnostic> {
        for statement in program {
            match &statement.kind {
                StmtKind::FunctionDeclaration {
                    return_type,
                    name,
                    parameters,
                    body,
                } => {
                    self.functions.insert(
                        name.clone(),
                        Function {
                            return_type: return_type.clone(),
                            parameters: parameters.clone(),
                            body: body.clone(),
                        },
                    );
                }
                StmtKind::Import { module, items } => {
                    self.execute_import(module, items, statement)?;
                }
                StmtKind::StructDeclaration { name, fields } => {
                    self.structs.insert(
                        name.clone(),
                        StructDef {
                            fields: fields.clone(),
                        },
                    );
                }
                StmtKind::ClassDeclaration {
                    name,
                    fields,
                    methods,
                } => {
                    let mut method_map = HashMap::new();
                    for method in methods {
                        method_map.insert(method.name.clone(), method.clone());
                    }
                    self.classes.insert(
                        name.clone(),
                        ClassDef {
                            fields: fields.clone(),
                            methods: method_map,
                        },
                    );
                }
                _ => {}
            }
        }
        for statement in program {
            if !matches!(
                statement.kind,
                StmtKind::FunctionDeclaration { .. }
                    | StmtKind::StructDeclaration { .. }
                    | StmtKind::ClassDeclaration { .. }
                    | StmtKind::Import { .. }
            ) {
                match self.execute(statement)? {
                    Flow::Normal => {}
                    _ => {
                        return Err(runtime_stmt(
                            statement,
                            "unexpected control flow at top level",
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn execute_import(
        &mut self,
        module_specifier: &str,
        items: &crate::ast::ImportItems,
        statement: &Stmt,
    ) -> Result<(), Diagnostic> {
        let module_instance = if let Some(m) = self.loaded_modules.get(module_specifier).cloned() {
            m
        } else {
            let (key, source, file_dir) = crate::module::load_module_source(
                module_specifier,
                self.current_file_dir.as_deref(),
                statement.span,
            )?;
            let module_ast = crate::module::parse_module(&source, statement.span)?;
            let mut sub_interpreter = Interpreter::new();
            sub_interpreter.current_file_dir = file_dir.or_else(|| self.current_file_dir.clone());
            sub_interpreter.loaded_modules = self.loaded_modules.clone();
            sub_interpreter.load_prelude()?;
            sub_interpreter.execute_program(&module_ast)?;
            self.loaded_modules = sub_interpreter.loaded_modules;
            let mod_inst = Rc::new(RefCell::new(ModuleInstance {
                name: module_specifier.to_string(),
                functions: sub_interpreter.functions,
                structs: sub_interpreter.structs,
                classes: sub_interpreter.classes,
                globals: sub_interpreter
                    .scopes
                    .into_iter()
                    .next()
                    .unwrap_or_default(),
            }));
            self.loaded_modules
                .insert(module_specifier.to_string(), Rc::clone(&mod_inst));
            self.loaded_modules.insert(key, Rc::clone(&mod_inst));
            mod_inst
        };

        match items {
            crate::ast::ImportItems::Module { alias } => {
                let default_alias = module_specifier
                    .split('.')
                    .next_back()
                    .unwrap_or(module_specifier);
                let effective_alias = alias.as_deref().unwrap_or(default_alias);
                self.scopes.last_mut().unwrap().insert(
                    effective_alias.to_string(),
                    Binding {
                        variable_type: Type::Module,
                        is_final: true,
                        value: Value::Module(Rc::clone(&module_instance)),
                    },
                );
            }
            crate::ast::ImportItems::All => {
                let m = module_instance.borrow();
                for (name, func) in &m.functions {
                    self.functions.insert(name.clone(), func.clone());
                }
                for (name, st) in &m.structs {
                    self.structs.insert(name.clone(), st.clone());
                }
                for (name, cl) in &m.classes {
                    self.classes.insert(name.clone(), cl.clone());
                }
                for (name, binding) in &m.globals {
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), binding.clone());
                }
            }
            crate::ast::ImportItems::Named(item_list) => {
                let m = module_instance.borrow();
                for item in item_list {
                    let target_name = item.alias.as_ref().unwrap_or(&item.name);
                    if let Some(cl) = m.classes.get(&item.name) {
                        self.classes.insert(target_name.clone(), cl.clone());
                        if target_name != &item.name {
                            self.type_aliases
                                .insert(target_name.clone(), item.name.clone());
                        }
                    }
                    if let Some(st) = m.structs.get(&item.name) {
                        self.structs.insert(target_name.clone(), st.clone());
                        if target_name != &item.name {
                            self.type_aliases
                                .insert(target_name.clone(), item.name.clone());
                        }
                    }
                    if let Some(func) = m.functions.get(&item.name) {
                        self.functions.insert(target_name.clone(), func.clone());
                    }
                    if let Some(binding) = m.globals.get(&item.name) {
                        self.scopes
                            .last_mut()
                            .unwrap()
                            .insert(target_name.clone(), binding.clone());
                    }
                }
            }
        }
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

    pub(super) fn get(&self, name: &str) -> Option<&Binding> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }

    pub(super) fn get_mut(&mut self, name: &str) -> Option<&mut Binding> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }

    pub(super) fn emit(&mut self, text: &str) -> Result<(), Diagnostic> {
        if self.stream_output {
            print!("{text}");
            io::stdout().flush().map_err(|error| {
                Diagnostic::without_span(Phase::Io, format!("could not write output: {error}"))
            })?;
        } else {
            self.output.push_str(text);
        }
        Ok(())
    }
}

pub(super) fn runtime_expr(expression: &Expr, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(Phase::Runtime, message, expression.span)
}

pub(super) fn runtime_stmt(statement: &Stmt, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(Phase::Runtime, message, statement.span)
}
