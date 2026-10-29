use super::Parser;
use crate::{
    ast::{FieldDef, ImportItem, ImportItems, MethodDef, Parameter, Stmt, StmtKind, Type},
    diagnostic::{Diagnostic, Phase},
    lexer::TokenKind,
    source::Span,
};

impl<'a> Parser<'a> {
    pub(super) fn declaration(&mut self) -> Result<Stmt, Diagnostic> {
        if self.matches(&[TokenKind::Import]) {
            return self.import_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::From]) {
            return self.parse_from_import_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::Struct]) {
            return self.struct_declaration(self.previous().span);
        }
        if self.matches(&[TokenKind::Class]) {
            return self.class_declaration(self.previous().span);
        }
        if self.matches(&[TokenKind::Def]) {
            return self.function_declaration(self.previous().span);
        }
        if self.matches(&[TokenKind::Final]) {
            return self.variable_declaration(true);
        }
        if self.is_type_start() {
            return self.variable_declaration(false);
        }
        self.statement()
    }

    fn parse_module_path(&mut self) -> Result<(String, Span), Diagnostic> {
        if self.matches(&[TokenKind::StringLiteral]) {
            let tok = self.previous().clone();
            return Ok((tok.lexeme, tok.span));
        }
        let first = self
            .consume(TokenKind::Identifier, "expected module name")?
            .clone();
        let mut path = first.lexeme;
        let mut end = first.span;
        while self.matches(&[TokenKind::Dot]) {
            let next_ident = self
                .consume(TokenKind::Identifier, "expected submodule name after '.'")?
                .clone();
            path.push('.');
            path.push_str(&next_ident.lexeme);
            end = next_ident.span;
        }
        Ok((path, first.span.merge(end)))
    }

    fn import_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let (module, _) = self.parse_module_path()?;
        let alias = if self.matches(&[TokenKind::As]) {
            Some(
                self.consume(TokenKind::Identifier, "expected alias name after 'as'")?
                    .lexeme
                    .clone(),
            )
        } else {
            None
        };
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after import statement")?
            .span;
        Ok(Stmt {
            kind: StmtKind::Import {
                module,
                items: ImportItems::Module { alias },
            },
            span: start.merge(end),
        })
    }

    fn parse_from_import_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let (module, _) = self.parse_module_path()?;
        self.consume(
            TokenKind::Import,
            "expected 'import' after module path in 'from' statement",
        )?;
        let items = if self.matches(&[TokenKind::Star]) {
            ImportItems::All
        } else {
            let closing_token = if self.matches(&[TokenKind::LeftParen]) {
                Some(TokenKind::RightParen)
            } else if self.matches(&[TokenKind::LeftBrace]) {
                Some(TokenKind::RightBrace)
            } else {
                None
            };
            let mut list = Vec::new();
            loop {
                let item_start = self.peek().span;
                let name = self
                    .consume(TokenKind::Identifier, "expected import symbol name")?
                    .clone();
                let (alias, item_end) = if self.matches(&[TokenKind::As]) {
                    let alias_tok = self
                        .consume(TokenKind::Identifier, "expected alias name after 'as'")?
                        .clone();
                    (Some(alias_tok.lexeme), alias_tok.span)
                } else {
                    (None, name.span)
                };
                list.push(ImportItem {
                    name: name.lexeme,
                    alias,
                    span: item_start.merge(item_end),
                });
                if !self.matches(&[TokenKind::Comma]) {
                    break;
                }
                if let Some(closing) = closing_token
                    && self.check(closing)
                {
                    break;
                }
            }
            if let Some(closing) = closing_token {
                self.consume(closing, "expected closing delimiter for import list")?;
            }
            ImportItems::Named(list)
        };
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after import statement")?
            .span;
        Ok(Stmt {
            kind: StmtKind::Import { module, items },
            span: start.merge(end),
        })
    }

    fn variable_declaration(&mut self, is_final: bool) -> Result<Stmt, Diagnostic> {
        let start = if is_final {
            self.previous().span
        } else {
            self.peek().span
        };
        let variable_type = self.parse_type(false)?;
        let name = self
            .consume(TokenKind::Identifier, "expected a variable name")?
            .clone();
        self.consume(TokenKind::Assign, "expected '=' after the variable name")?;
        let initializer = self.expression()?;
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after the declaration")?
            .span;
        Ok(Stmt {
            kind: StmtKind::VariableDeclaration {
                is_final,
                variable_type,
                name: name.lexeme,
                initializer,
            },
            span: start.merge(end),
        })
    }

    fn function_declaration(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let (return_type, name) = if self.peek().kind == TokenKind::Identifier
            && self.peek_next().kind == TokenKind::LeftParen
        {
            let name_tok = self.advance().clone();
            (Type::Void, name_tok)
        } else {
            let return_type = self.parse_type(true)?;
            let name_tok = self
                .consume(TokenKind::Identifier, "expected a function name")?
                .clone();
            (return_type, name_tok)
        };
        self.consume(TokenKind::LeftParen, "expected '(' after the function name")?;
        let mut parameters = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error_here("a function cannot have more than 255 parameters"));
                }
                let parameter_start = self.peek().span;
                let parameter_type = self.parse_type(false)?;
                let parameter_name = self
                    .consume(TokenKind::Identifier, "expected a parameter name")?
                    .clone();
                parameters.push(Parameter {
                    parameter_type,
                    name: parameter_name.lexeme,
                    span: parameter_start.merge(parameter_name.span),
                });
                if !self.matches(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RightParen, "expected ')' after the parameters")?;
        self.consume(
            TokenKind::LeftBrace,
            "expected '{' before the function body",
        )?;
        let (body, end) = self.block_contents()?;
        Ok(Stmt {
            kind: StmtKind::FunctionDeclaration {
                return_type,
                name: name.lexeme,
                parameters,
                body,
            },
            span: start.merge(end),
        })
    }

    fn struct_declaration(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let name = self
            .consume(TokenKind::Identifier, "expected a struct name")?
            .clone();
        self.consume(TokenKind::LeftBrace, "expected '{' after struct name")?;
        let mut fields = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            if self.check(TokenKind::Def) {
                return Err(Diagnostic::new(
                    Phase::Parser,
                    "structs cannot contain methods; use 'class' instead",
                    self.peek().span,
                ));
            }
            let field_start = self.peek().span;
            let field_type = self.parse_type(false)?;
            let field_name = self
                .consume(TokenKind::Identifier, "expected a field name")?
                .clone();
            if !self.matches(&[TokenKind::Semicolon, TokenKind::Comma]) {
                self.consume(TokenKind::Semicolon, "expected ';' after field declaration")?;
            }
            fields.push(FieldDef {
                field_type,
                name: field_name.lexeme,
                span: field_start.merge(field_name.span),
            });
        }
        let end = self
            .consume(TokenKind::RightBrace, "expected '}' after struct body")?
            .span;
        Ok(Stmt {
            kind: StmtKind::StructDeclaration {
                name: name.lexeme,
                fields,
            },
            span: start.merge(end),
        })
    }

    fn class_declaration(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let name = self
            .consume(TokenKind::Identifier, "expected a class name")?
            .clone();
        self.consume(TokenKind::LeftBrace, "expected '{' after class name")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            if self.matches(&[TokenKind::Def]) {
                let method_start = self.previous().span;
                let method = self.method_declaration(method_start)?;
                methods.push(method);
            } else {
                let field_start = self.peek().span;
                let field_type = self.parse_type(false)?;
                let field_name = self
                    .consume(TokenKind::Identifier, "expected a field name")?
                    .clone();
                if !self.matches(&[TokenKind::Semicolon, TokenKind::Comma]) {
                    self.consume(TokenKind::Semicolon, "expected ';' after field declaration")?;
                }
                fields.push(FieldDef {
                    field_type,
                    name: field_name.lexeme,
                    span: field_start.merge(field_name.span),
                });
            }
        }
        let end = self
            .consume(TokenKind::RightBrace, "expected '}' after class body")?
            .span;
        Ok(Stmt {
            kind: StmtKind::ClassDeclaration {
                name: name.lexeme,
                fields,
                methods,
            },
            span: start.merge(end),
        })
    }

    fn method_declaration(&mut self, start: Span) -> Result<MethodDef, Diagnostic> {
        let (return_type, name) = if self.peek().kind == TokenKind::Identifier
            && self.peek_next().kind == TokenKind::LeftParen
        {
            let name_tok = self.advance().clone();
            (Type::Void, name_tok)
        } else {
            let return_type = self.parse_type(true)?;
            let name_tok = self
                .consume(TokenKind::Identifier, "expected a method name")?
                .clone();
            (return_type, name_tok)
        };
        self.consume(TokenKind::LeftParen, "expected '(' after the method name")?;
        let mut parameters = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(self.error_here("a method cannot have more than 255 parameters"));
                }
                let parameter_start = self.peek().span;
                let parameter_type = self.parse_type(false)?;
                let parameter_name = self
                    .consume(TokenKind::Identifier, "expected a parameter name")?
                    .clone();
                parameters.push(Parameter {
                    parameter_type,
                    name: parameter_name.lexeme,
                    span: parameter_start.merge(parameter_name.span),
                });
                if !self.matches(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RightParen, "expected ')' after the parameters")?;
        self.consume(TokenKind::LeftBrace, "expected '{' before the method body")?;
        let (body, end) = self.block_contents()?;
        Ok(MethodDef {
            return_type,
            name: name.lexeme,
            parameters,
            body,
            span: start.merge(end),
        })
    }

    fn parse_type(&mut self, allow_void: bool) -> Result<Type, Diagnostic> {
        let token = self.advance().clone();
        let mut value = match token.kind {
            TokenKind::Int => Type::Int,
            TokenKind::Float => Type::Float,
            TokenKind::Bool => Type::Bool,
            TokenKind::StringType => Type::String,
            TokenKind::Void if allow_void => Type::Void,
            TokenKind::Any => Type::Any,
            TokenKind::Dict => Type::dict(),
            TokenKind::Identifier => Type::Custom(token.lexeme),
            TokenKind::Map => {
                self.consume(TokenKind::Less, "expected '<' after 'map'")?;
                let key = self.parse_type(false)?;
                self.consume(
                    TokenKind::Comma,
                    "expected ',' between map key and value types",
                )?;
                let val = self.parse_type(false)?;
                self.consume(TokenKind::Greater, "expected '>' after map value type")?;
                Type::map(key, val)
            }
            _ => {
                return Err(Diagnostic::new(
                    Phase::Parser,
                    "expected a type",
                    token.span,
                ));
            }
        };
        while self.matches(&[TokenKind::LeftBracket]) {
            self.consume(TokenKind::RightBracket, "expected ']' in array type")?;
            if value == Type::Void {
                return Err(Diagnostic::new(
                    Phase::Parser,
                    "void arrays are not allowed",
                    token.span,
                ));
            }
            value = Type::array(value);
        }
        Ok(value)
    }

    pub(super) fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        if self.matches(&[TokenKind::LeftBrace]) {
            let start = self.previous().span;
            let (statements, end) = self.block_contents()?;
            return Ok(Stmt {
                kind: StmtKind::Block(statements),
                span: start.merge(end),
            });
        }
        if self.matches(&[TokenKind::If]) {
            return self.if_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::While]) {
            return self.while_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::For]) {
            return self.for_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::Break]) {
            return self.keyword_statement(StmtKind::Break, "break");
        }
        if self.matches(&[TokenKind::Continue]) {
            return self.keyword_statement(StmtKind::Continue, "continue");
        }
        if self.matches(&[TokenKind::Return]) {
            return self.return_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::Print]) {
            return self.print_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::Try]) {
            return self.try_statement(self.previous().span);
        }
        if self.matches(&[TokenKind::Throw]) {
            return self.throw_statement(self.previous().span);
        }
        self.expression_statement()
    }

    fn block_contents(&mut self) -> Result<(Vec<Stmt>, Span), Diagnostic> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        let end = self
            .consume(TokenKind::RightBrace, "expected '}' after the block")?
            .span;
        Ok((statements, end))
    }

    fn try_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        self.consume(TokenKind::LeftBrace, "expected '{' after 'try'")?;
        let (try_body, _) = self.block_contents()?;

        self.consume(TokenKind::Catch, "expected 'catch' after 'try' block")?;

        let (catch_var_type, catch_var_name) = if self.matches(&[TokenKind::LeftParen]) {
            let (ty, name) = if self.is_type_start() {
                let ty = self.parse_type(false)?;
                let name = self
                    .consume(
                        TokenKind::Identifier,
                        "expected variable name after type in catch",
                    )?
                    .lexeme
                    .clone();
                (Some(ty), name)
            } else {
                let name = self
                    .consume(TokenKind::Identifier, "expected variable name in catch")?
                    .lexeme
                    .clone();
                (None, name)
            };
            self.consume(TokenKind::RightParen, "expected ')' after catch variable")?;
            (ty, name)
        } else if self.check(TokenKind::LeftBrace) {
            (None, "error".to_string())
        } else if self.is_type_start() {
            let ty = self.parse_type(false)?;
            let name = self
                .consume(
                    TokenKind::Identifier,
                    "expected variable name after type in catch",
                )?
                .lexeme
                .clone();
            (Some(ty), name)
        } else {
            let name = self
                .consume(TokenKind::Identifier, "expected variable name in catch")?
                .lexeme
                .clone();
            (None, name)
        };

        self.consume(TokenKind::LeftBrace, "expected '{' before catch block body")?;
        let (catch_body, end) = self.block_contents()?;

        Ok(Stmt {
            kind: StmtKind::TryCatch {
                try_body,
                catch_var_type,
                catch_var_name,
                catch_body,
            },
            span: start.merge(end),
        })
    }

    fn throw_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let expr = self.expression()?;
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after throw expression")?
            .span;
        Ok(Stmt {
            kind: StmtKind::Throw(expr),
            span: start.merge(end),
        })
    }

    fn if_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let parenthesized = self.matches(&[TokenKind::LeftParen]);
        let condition = self.expression()?;
        if parenthesized {
            self.consume(TokenKind::RightParen, "expected ')' after the condition")?;
        }
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        let end = else_branch
            .as_ref()
            .map_or(then_branch.span, |branch| branch.span);
        Ok(Stmt {
            kind: StmtKind::If {
                condition,
                then_branch,
                else_branch,
            },
            span: start.merge(end),
        })
    }

    fn while_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let parenthesized = self.matches(&[TokenKind::LeftParen]);
        let condition = self.expression()?;
        if parenthesized {
            self.consume(TokenKind::RightParen, "expected ')' after the condition")?;
        }
        let body = Box::new(self.statement()?);
        let span = start.merge(body.span);
        Ok(Stmt {
            kind: StmtKind::While { condition, body },
            span,
        })
    }

    fn for_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let is_paren_for_in = self.check(TokenKind::LeftParen)
            && self
                .tokens
                .get(self.current + 1)
                .is_some_and(|t| t.kind == TokenKind::Identifier)
            && self
                .tokens
                .get(self.current + 2)
                .is_some_and(|t| t.kind == TokenKind::In);

        if is_paren_for_in || !self.matches(&[TokenKind::LeftParen]) {
            if is_paren_for_in {
                self.advance();
            }
            let variable = self
                .consume(
                    TokenKind::Identifier,
                    "expected a loop variable after 'for'",
                )?
                .lexeme
                .clone();
            self.consume(TokenKind::In, "expected 'in' after the loop variable")?;
            let iterable = self.expression()?;
            if is_paren_for_in {
                self.consume(
                    TokenKind::RightParen,
                    "expected ')' after 'for (... in ...)'",
                )?;
            }
            let body = Box::new(self.statement()?);
            let span = start.merge(body.span);
            return Ok(Stmt {
                kind: StmtKind::ForEach {
                    variable,
                    iterable,
                    body,
                },
                span,
            });
        }
        let initializer = if self.matches(&[TokenKind::Semicolon]) {
            None
        } else if self.matches(&[TokenKind::Final]) {
            Some(Box::new(self.variable_declaration(true)?))
        } else if self.is_type_start() {
            Some(Box::new(self.variable_declaration(false)?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };
        let condition = if self.check(TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(
            TokenKind::Semicolon,
            "expected ';' after the loop condition",
        )?;
        let increment = if self.check(TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenKind::RightParen, "expected ')' after the for clauses")?;
        let body = Box::new(self.statement()?);
        let span = start.merge(body.span);
        Ok(Stmt {
            kind: StmtKind::For {
                initializer,
                condition,
                increment,
                body,
            },
            span,
        })
    }

    fn return_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        let value = if self.check(TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after return")?
            .span;
        Ok(Stmt {
            kind: StmtKind::Return(value),
            span: start.merge(end),
        })
    }

    fn keyword_statement(&mut self, kind: StmtKind, keyword: &str) -> Result<Stmt, Diagnostic> {
        let start = self.previous().span;
        let end = self
            .consume(
                TokenKind::Semicolon,
                &format!("expected ';' after '{keyword}'"),
            )?
            .span;
        Ok(Stmt {
            kind,
            span: start.merge(end),
        })
    }

    fn print_statement(&mut self, start: Span) -> Result<Stmt, Diagnostic> {
        self.consume(TokenKind::LeftParen, "expected '(' after 'print'")?;
        let mut arguments = Vec::new();
        let mut separator = None;
        let mut end_value = None;
        let mut saw_named = false;

        while !self.check(TokenKind::RightParen) {
            let is_named =
                self.check(TokenKind::Identifier) && self.peek_next().kind == TokenKind::Assign;
            if is_named {
                saw_named = true;
                let name = self.advance().clone();
                self.advance();
                let value = self.expression()?;
                match name.lexeme.as_str() {
                    "sep" if separator.is_none() => separator = Some(value),
                    "end" if end_value.is_none() => end_value = Some(value),
                    "sep" | "end" => {
                        return Err(Diagnostic::new(
                            Phase::Parser,
                            format!(
                                "print argument '{}' was provided more than once",
                                name.lexeme
                            ),
                            name.span,
                        ));
                    }
                    _ => {
                        return Err(Diagnostic::new(
                            Phase::Parser,
                            format!("unknown named print argument '{}'", name.lexeme),
                            name.span,
                        ));
                    }
                }
            } else {
                if saw_named {
                    return Err(self.error_here(
                        "positional print arguments must come before named arguments",
                    ));
                }
                arguments.push(self.expression()?);
            }
            if !self.matches(&[TokenKind::Comma]) {
                break;
            }
        }

        self.consume(TokenKind::RightParen, "expected ')' after print arguments")?;
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after print")?
            .span;
        Ok(Stmt {
            kind: StmtKind::Print {
                arguments,
                separator,
                end: end_value,
            },
            span: start.merge(end),
        })
    }

    fn expression_statement(&mut self) -> Result<Stmt, Diagnostic> {
        let expression = self.expression()?;
        let end = self
            .consume(TokenKind::Semicolon, "expected ';' after the expression")?
            .span;
        let span = expression.span.merge(end);
        Ok(Stmt {
            kind: StmtKind::Expression(expression),
            span,
        })
    }
}
