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
    rc::Rc,
};
use value::ClassInstance;

#[derive(Debug, Clone)]
pub(super) struct Binding {
    pub(super) variable_type: Type,
    pub(super) is_final: bool,
    pub(super) value: Value,
}

#[derive(Debug, Clone)]
pub(super) struct Function {
    pub(super) return_type: Type,
    pub(super) parameters: Vec<Parameter>,
    pub(super) body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub(super) struct StructDef {
    pub(super) fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub(super) struct ClassDef {
    pub(super) fields: Vec<FieldDef>,
    pub(super) methods: HashMap<String, MethodDef>,
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
    pub(super) current_instance: Option<Rc<RefCell<ClassInstance>>>,
    pub(super) output: String,
    pub(super) print_calls: Vec<String>,
    pub(super) call_depth: usize,
    pub(super) input: Input,
    pub(super) stream_output: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            ..Self::default()
        }
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
        self.execute_program(program)?;
        Ok(self.print_calls)
    }

    pub fn interpret_text(mut self, program: &Program) -> Result<String, Diagnostic> {
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
