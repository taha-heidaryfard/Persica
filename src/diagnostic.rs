use crate::source::Span;
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Io,
    Lexer,
    Parser,
    Semantic,
    Runtime,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Io => "io",
            Self::Lexer => "lexer",
            Self::Parser => "parser",
            Self::Semantic => "semantic",
            Self::Runtime => "runtime",
        };
        f.write_str(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub phase: Phase,
    pub message: String,
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(phase: Phase, message: impl Into<String>, span: Span) -> Self {
        Self {
            phase,
            message: message.into(),
            span: Some(span),
        }
    }

    pub fn without_span(phase: Phase, message: impl Into<String>) -> Self {
        Self {
            phase,
            message: message.into(),
            span: None,
        }
    }

    pub fn render(&self, file_name: &str, source: &str) -> String {
        let Some(span) = self.span else {
            return format!("{} error: {}", self.phase, self.message);
        };

        let line_text = source
            .lines()
            .nth(span.line.saturating_sub(1))
            .unwrap_or("");
        let width = source
            .get(span.start..span.end)
            .map(|text| text.chars().count().max(1))
            .unwrap_or(1);
        let marker = format!(
            "{}{}",
            " ".repeat(span.column.saturating_sub(1)),
            "^".repeat(width)
        );

        format!(
            "{} error: {}\n --> {}:{}:{}\n  |\n{:>2} | {}\n  | {}",
            self.phase,
            self.message,
            file_name,
            span.line,
            span.column,
            span.line,
            line_text,
            marker
        )
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(f, "{} error at {}: {}", self.phase, span, self.message),
            None => write!(f, "{} error: {}", self.phase, self.message),
        }
    }
}

impl Error for Diagnostic {}
