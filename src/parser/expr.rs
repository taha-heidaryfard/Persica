use super::Parser;
use crate::{
    ast::{BinaryOp, Expr, ExprKind, Literal, UnaryOp},
    diagnostic::{Diagnostic, Phase},
    lexer::TokenKind,
    source::Span,
};

impl<'a> Parser<'a> {
    pub(super) fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Diagnostic> {
        let expression = self.logical_or()?;
        let assignment = if self.matches(&[TokenKind::Assign]) {
            Some(None)
        } else if self.matches(&[TokenKind::PlusAssign]) {
            Some(Some(BinaryOp::Add))
        } else if self.matches(&[TokenKind::MinusAssign]) {
            Some(Some(BinaryOp::Subtract))
        } else if self.matches(&[TokenKind::StarAssign]) {
            Some(Some(BinaryOp::Multiply))
        } else if self.matches(&[TokenKind::SlashAssign]) {
            Some(Some(BinaryOp::Divide))
        } else if self.matches(&[TokenKind::ModuloAssign]) {
            Some(Some(BinaryOp::Modulo))
        } else {
            None
        };
        let Some(compound_op) = assignment else {
            return Ok(expression);
        };
        let equals = self.previous().span;
        let raw_value = self.assignment()?;
        let value = if let Some(op) = compound_op {
            binary(expression.clone(), op, raw_value)
        } else {
            raw_value
        };
        let span = expression.span.merge(value.span);
        match expression.kind {
            ExprKind::Identifier(name) => Ok(Expr {
                kind: ExprKind::Assignment {
                    name,
                    value: Box::new(value),
                },
                span,
            }),
            ExprKind::Index { target, index } => Ok(Expr {
                kind: ExprKind::IndexAssignment {
                    target,
                    index,
                    value: Box::new(value),
                },
                span,
            }),
            ExprKind::Get { object, name } => Ok(Expr {
                kind: ExprKind::Set {
                    object,
                    name,
                    value: Box::new(value),
                },
                span,
            }),
            _ => Err(Diagnostic::new(
                Phase::Parser,
                "invalid assignment target",
                equals,
            )),
        }
    }

    fn logical_or(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(Self::logical_and, &[(TokenKind::Or, BinaryOp::Or)])
    }

    fn logical_and(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(Self::equality, &[(TokenKind::And, BinaryOp::And)])
    }

    fn equality(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(
            Self::comparison,
            &[
                (TokenKind::Equal, BinaryOp::Equal),
                (TokenKind::NotEqual, BinaryOp::NotEqual),
            ],
        )
    }

    fn comparison(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(
            Self::term,
            &[
                (TokenKind::Greater, BinaryOp::Greater),
                (TokenKind::GreaterEqual, BinaryOp::GreaterEqual),
                (TokenKind::Less, BinaryOp::Less),
                (TokenKind::LessEqual, BinaryOp::LessEqual),
            ],
        )
    }

    fn term(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(
            Self::factor,
            &[
                (TokenKind::Plus, BinaryOp::Add),
                (TokenKind::Minus, BinaryOp::Subtract),
            ],
        )
    }

    fn factor(&mut self) -> Result<Expr, Diagnostic> {
        self.binary_level(
            Self::unary,
            &[
                (TokenKind::Star, BinaryOp::Multiply),
                (TokenKind::Slash, BinaryOp::Divide),
                (TokenKind::Modulo, BinaryOp::Modulo),
            ],
        )
    }

    fn binary_level(
        &mut self,
        next: fn(&mut Self) -> Result<Expr, Diagnostic>,
        operators: &[(TokenKind, BinaryOp)],
    ) -> Result<Expr, Diagnostic> {
        let mut expression = next(self)?;
        while let Some((_, op)) = operators.iter().find(|(kind, _)| self.check(*kind)) {
            self.advance();
            expression = binary(expression, *op, next(self)?);
        }
        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        let op = if self.matches(&[TokenKind::Not]) {
            Some(UnaryOp::Not)
        } else if self.matches(&[TokenKind::Minus]) {
            Some(UnaryOp::Negate)
        } else {
            None
        };
        if let Some(op) = op {
            let start = self.previous().span;
            let right = self.unary()?;
            let span = start.merge(right.span);
            return Ok(Expr {
                kind: ExprKind::Unary {
                    op,
                    right: Box::new(right),
                },
                span,
            });
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, Diagnostic> {
        let mut expression = self.primary()?;
        loop {
            if self.matches(&[TokenKind::LeftParen]) {
                let ExprKind::Identifier(callee) = expression.kind else {
                    return Err(Diagnostic::new(
                        Phase::Parser,
                        "only named functions can be called",
                        expression.span,
                    ));
                };
                let mut arguments = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    loop {
                        if arguments.len() >= 255 {
                            return Err(
                                self.error_here("a call cannot have more than 255 arguments")
                            );
                        }
                        arguments.push(self.expression()?);
                        if !self.matches(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }
                let end = self
                    .consume(TokenKind::RightParen, "expected ')' after arguments")?
                    .span;
                expression = Expr {
                    kind: ExprKind::Call { callee, arguments },
                    span: expression.span.merge(end),
                };
            } else if self.matches(&[TokenKind::LeftBracket]) {
                let index = self.expression()?;
                let end = self
                    .consume(TokenKind::RightBracket, "expected ']' after index")?
                    .span;
                let span = expression.span.merge(end);
                expression = Expr {
                    kind: ExprKind::Index {
                        target: Box::new(expression),
                        index: Box::new(index),
                    },
                    span,
                };
            } else if self.matches(&[TokenKind::Dot]) {
                let name_token = self
                    .consume(
                        TokenKind::Identifier,
                        "expected property or method name after '.'",
                    )?
                    .clone();
                if self.matches(&[TokenKind::LeftParen]) {
                    let mut arguments = Vec::new();
                    if !self.check(TokenKind::RightParen) {
                        loop {
                            if arguments.len() >= 255 {
                                return Err(self.error_here(
                                    "a method call cannot have more than 255 arguments",
                                ));
                            }
                            arguments.push(self.expression()?);
                            if !self.matches(&[TokenKind::Comma]) {
                                break;
                            }
                        }
                    }
                    let end = self
                        .consume(TokenKind::RightParen, "expected ')' after method arguments")?
                        .span;
                    expression = Expr {
                        span: expression.span.merge(end),
                        kind: ExprKind::MethodCall {
                            object: Box::new(expression),
                            method: name_token.lexeme,
                            arguments,
                        },
                    };
                } else {
                    let span = expression.span.merge(name_token.span);
                    expression = Expr {
                        kind: ExprKind::Get {
                            object: Box::new(expression),
                            name: name_token.lexeme,
                        },
                        span,
                    };
                }
            } else if self.matches(&[TokenKind::Increment, TokenKind::Decrement]) {
                let op_token = self.previous().clone();
                let op = if op_token.kind == TokenKind::Increment {
                    BinaryOp::Add
                } else {
                    BinaryOp::Subtract
                };
                let ExprKind::Identifier(name) = expression.kind else {
                    return Err(Diagnostic::new(
                        Phase::Parser,
                        "increment target must be a variable",
                        op_token.span,
                    ));
                };
                let left = Expr {
                    kind: ExprKind::Identifier(name.clone()),
                    span: expression.span,
                };
                let one = literal(Literal::Int(1), op_token.span);
                let value = binary(left, op, one);
                expression = Expr {
                    kind: ExprKind::Assignment {
                        name,
                        value: Box::new(value),
                    },
                    span: expression.span.merge(op_token.span),
                };
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        if self.matches(&[TokenKind::Number]) {
            let token = self.previous().clone();
            if token.lexeme.contains('.') {
                let value = token.lexeme.parse::<f64>().map_err(|_| {
                    Diagnostic::new(Phase::Parser, "invalid float literal", token.span)
                })?;
                return Ok(literal(Literal::Float(value), token.span));
            }
            let value = token.lexeme.parse::<i64>().map_err(|_| {
                Diagnostic::new(Phase::Parser, "integer literal is out of range", token.span)
            })?;
            return Ok(literal(Literal::Int(value), token.span));
        }
        if self.matches(&[TokenKind::StringLiteral]) {
            let token = self.previous().clone();
            return Ok(literal(Literal::String(token.lexeme), token.span));
        }
        if self.matches(&[TokenKind::True, TokenKind::False]) {
            let token = self.previous().clone();
            return Ok(literal(
                Literal::Bool(token.kind == TokenKind::True),
                token.span,
            ));
        }
        if self.matches(&[TokenKind::This]) {
            let token = self.previous().clone();
            return Ok(Expr {
                kind: ExprKind::This,
                span: token.span,
            });
        }
        if self.matches(&[TokenKind::Identifier]) {
            let token = self.previous().clone();
            return Ok(Expr {
                kind: ExprKind::Identifier(token.lexeme),
                span: token.span,
            });
        }
        if self.matches(&[TokenKind::LeftParen]) {
            let start = self.previous().span;
            let expression = self.expression()?;
            let end = self
                .consume(TokenKind::RightParen, "expected ')' after the expression")?
                .span;
            return Ok(Expr {
                kind: ExprKind::Grouping(Box::new(expression)),
                span: start.merge(end),
            });
        }
        if self.matches(&[TokenKind::LeftBracket]) {
            let start = self.previous().span;
            let mut elements = Vec::new();
            if !self.check(TokenKind::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.matches(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            let end = self
                .consume(TokenKind::RightBracket, "expected ']' after array elements")?
                .span;
            return Ok(Expr {
                kind: ExprKind::Array(elements),
                span: start.merge(end),
            });
        }
        if self.matches(&[TokenKind::LeftBrace]) {
            let start = self.previous().span;
            let mut entries = Vec::new();
            if !self.check(TokenKind::RightBrace) {
                loop {
                    let key = self.expression()?;
                    self.consume(TokenKind::Colon, "expected ':' after map key")?;
                    let value = self.expression()?;
                    entries.push((key, value));
                    if !self.matches(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            let end = self
                .consume(TokenKind::RightBrace, "expected '}' after map entries")?
                .span;
            return Ok(Expr {
                kind: ExprKind::Map(entries),
                span: start.merge(end),
            });
        }
        Err(self.error_here("expected an expression"))
    }
}

pub(super) fn literal(value: Literal, span: Span) -> Expr {
    Expr {
        kind: ExprKind::Literal(value),
        span,
    }
}

pub(super) fn binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    let span = left.span.merge(right.span);
    Expr {
        kind: ExprKind::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        },
        span,
    }
}
