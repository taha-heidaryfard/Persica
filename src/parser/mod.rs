mod expr;
mod stmt;

use crate::{
    ast::Program,
    diagnostic::{Diagnostic, Phase},
    lexer::{Token, TokenKind},
};

pub fn parse(tokens: &[Token]) -> Result<Program, Diagnostic> {
    if tokens.is_empty() {
        return Err(Diagnostic::without_span(
            Phase::Parser,
            "token stream is empty",
        ));
    }
    Parser { tokens, current: 0 }.parse()
}

pub(super) struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn parse(mut self) -> Result<Program, Diagnostic> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    pub(super) fn is_type_start(&self) -> bool {
        if matches!(
            self.peek().kind,
            TokenKind::Int
                | TokenKind::Float
                | TokenKind::Bool
                | TokenKind::StringType
                | TokenKind::Map
                | TokenKind::Dict
                | TokenKind::Any
        ) {
            return true;
        }
        self.is_custom_type_declaration()
    }

    pub(super) fn is_custom_type_declaration(&self) -> bool {
        if self.peek().kind != TokenKind::Identifier {
            return false;
        }
        let mut idx = self.current + 1;
        while idx + 1 < self.tokens.len()
            && self.tokens[idx].kind == TokenKind::LeftBracket
            && self.tokens[idx + 1].kind == TokenKind::RightBracket
        {
            idx += 2;
        }
        idx < self.tokens.len() && self.tokens[idx].kind == TokenKind::Identifier
    }

    pub(super) fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        if kinds.iter().any(|kind| self.check(*kind)) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(super) fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, Diagnostic> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.error_here(message))
        }
    }

    pub(super) fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    pub(super) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub(super) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub(super) fn peek_next(&self) -> &Token {
        self.tokens
            .get(self.current + 1)
            .unwrap_or_else(|| self.tokens.last().expect("token stream is not empty"))
    }

    pub(super) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub(super) fn error_here(&self, message: impl Into<String>) -> Diagnostic {
        Diagnostic::new(Phase::Parser, message, self.peek().span)
    }
}
