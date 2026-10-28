mod builtins;
mod expr;
mod stmt;
mod symbol;

use crate::{
    ast::{Program, StmtKind, Type},
    diagnostic::Diagnostic,
};
use std::collections::HashMap;
use symbol::{FunctionSignature, VariableSymbol, error};

pub fn validate(program: &Program) -> Result<(), Diagnostic> {
    Analyzer::new().validate(program)
}

struct Analyzer {
    scopes: Vec<HashMap<String, VariableSymbol>>,
    functions: HashMap<String, FunctionSignature>,
    loop_depth: usize,
    current_return: Option<Type>,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            loop_depth: 0,
            current_return: None,
        }
    }

    fn validate(mut self, program: &Program) -> Result<(), Diagnostic> {
        self.collect_functions(program)?;
        for statement in program {
            self.statement(statement)?;
        }
        Ok(())
    }

    fn collect_functions(&mut self, program: &Program) -> Result<(), Diagnostic> {
        for statement in program {
            if let StmtKind::FunctionDeclaration {
                return_type,
                name,
                parameters,
                ..
            } = &statement.kind
            {
                if builtins::is_reserved(name.as_str()) {
                    return Err(error(
                        statement,
                        format!("'{name}' is a reserved built-in function"),
                    ));
                }
                if self.functions.contains_key(name) {
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
