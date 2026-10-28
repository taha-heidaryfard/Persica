mod token;

pub use token::{Token, TokenKind};

use crate::{
    diagnostic::{Diagnostic, Phase},
    source::Span,
};

pub fn tokenize(source: &str) -> Result<Vec<Token>, Diagnostic> {
    Lexer::new(source).tokenize()
}

struct Lexer<'a> {
    source: &'a str,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, Diagnostic> {
        while !self.is_at_end() {
            self.scan_token()?;
        }
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            span: Span::new(self.current, self.current, self.line, self.column),
        });
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), Diagnostic> {
        let start = self.current;
        let line = self.line;
        let column = self.column;
        let ch = self.advance().expect("lexer advanced past end");

        match ch {
            ' ' | '\t' | '\r' | '\n' => Ok(()),
            '(' => self.simple(TokenKind::LeftParen, start, line, column),
            ')' => self.simple(TokenKind::RightParen, start, line, column),
            '{' => self.simple(TokenKind::LeftBrace, start, line, column),
            '}' => self.simple(TokenKind::RightBrace, start, line, column),
            '[' => self.simple(TokenKind::LeftBracket, start, line, column),
            ']' => self.simple(TokenKind::RightBracket, start, line, column),
            ',' => self.simple(TokenKind::Comma, start, line, column),
            ';' => self.simple(TokenKind::Semicolon, start, line, column),
            '+' => {
                let kind = if self.match_char('+') {
                    TokenKind::Increment
                } else if self.match_char('=') {
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                };
                self.simple(kind, start, line, column)
            }
            '-' => {
                let kind = if self.match_char('-') {
                    TokenKind::Decrement
                } else if self.match_char('=') {
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                };
                self.simple(kind, start, line, column)
            }
            '*' => {
                let kind = if self.match_char('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                };
                self.simple(kind, start, line, column)
            }
            '/' if self.match_char('/') => {
                while self.peek().is_some_and(|c| c != '\n') {
                    self.advance();
                }
                Ok(())
            }
            '/' if self.match_char('*') => self.block_comment(start, line, column),
            '/' => {
                let kind = if self.match_char('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                };
                self.simple(kind, start, line, column)
            }
            '%' => {
                let kind = if self.match_char('=') {
                    TokenKind::ModuloAssign
                } else {
                    TokenKind::Modulo
                };
                self.simple(kind, start, line, column)
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                };
                self.simple(kind, start, line, column)
            }
            '!' => {
                let kind = if self.match_char('=') {
                    TokenKind::NotEqual
                } else {
                    TokenKind::Not
                };
                self.simple(kind, start, line, column)
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.simple(kind, start, line, column)
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.simple(kind, start, line, column)
            }
            '&' if self.match_char('&') => self.simple(TokenKind::And, start, line, column),
            '|' if self.match_char('|') => self.simple(TokenKind::Or, start, line, column),
            '&' | '|' => Err(self.error(start, line, column, format!("expected a second '{ch}'"))),
            '"' => self.string(start, line, column),
            c if c.is_ascii_digit() => self.number(start, line, column),
            c if is_identifier_start(c) => self.identifier(start, line, column),
            _ => Err(self.error(start, line, column, format!("unexpected character '{ch}'"))),
        }
    }

    fn simple(
        &mut self,
        kind: TokenKind,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<(), Diagnostic> {
        self.push(kind, start, line, column, None);
        Ok(())
    }

    fn number(&mut self, start: usize, line: usize, column: usize) -> Result<(), Diagnostic> {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        self.push(TokenKind::Number, start, line, column, None);
        Ok(())
    }

    fn identifier(&mut self, start: usize, line: usize, column: usize) -> Result<(), Diagnostic> {
        while self.peek().is_some_and(is_identifier_continue) {
            self.advance();
        }
        let text = &self.source[start..self.current];
        let kind = match text {
            "int" => TokenKind::Int,
            "bool" => TokenKind::Bool,
            "string" => TokenKind::StringType,
            "float" => TokenKind::Float,
            "void" => TokenKind::Void,
            "final" => TokenKind::Final,
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "print" => TokenKind::Print,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier,
        };
        self.push(kind, start, line, column, None);
        Ok(())
    }

    fn string(&mut self, start: usize, line: usize, column: usize) -> Result<(), Diagnostic> {
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                self.push(TokenKind::StringLiteral, start, line, column, Some(value));
                return Ok(());
            }
            if ch == '\n' {
                return Err(self.error(start, line, column, "unterminated string literal"));
            }
            self.advance();
            if ch == '\\' {
                let Some(escaped) = self.advance() else {
                    return Err(self.error(start, line, column, "unterminated string escape"));
                };
                value.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '"' => '"',
                    '\\' => '\\',
                    _ => {
                        return Err(self.error(
                            self.current - escaped.len_utf8() - 1,
                            self.line,
                            self.column.saturating_sub(2),
                            format!("unknown escape '\\{escaped}'"),
                        ));
                    }
                });
            } else {
                value.push(ch);
            }
        }
        Err(self.error(start, line, column, "unterminated string literal"))
    }

    fn block_comment(
        &mut self,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<(), Diagnostic> {
        let mut depth = 1usize;
        while !self.is_at_end() {
            if self.peek() == Some('/') && self.peek_next() == Some('*') {
                self.advance();
                self.advance();
                depth += 1;
            } else if self.peek() == Some('*') && self.peek_next() == Some('/') {
                self.advance();
                self.advance();
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            } else {
                self.advance();
            }
        }
        Err(self.error(start, line, column, "unterminated block comment"))
    }

    fn push(
        &mut self,
        kind: TokenKind,
        start: usize,
        line: usize,
        column: usize,
        lexeme: Option<String>,
    ) {
        self.tokens.push(Token {
            kind,
            lexeme: lexeme.unwrap_or_else(|| self.source[start..self.current].to_owned()),
            span: Span::new(start, self.current, line, column),
        });
    }

    fn error(
        &self,
        start: usize,
        line: usize,
        column: usize,
        message: impl Into<String>,
    ) -> Diagnostic {
        Diagnostic::new(
            Phase::Lexer,
            message,
            Span::new(start, self.current.max(start + 1), line, column),
        )
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current..)?.chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source.get(self.current..)?.chars();
        chars.next()?;
        chars.next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.current += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_numeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_unicode_identifiers_and_comments() {
        let tokens = tokenize("int سن = 20; // age\nprint(سن);").unwrap();
        assert_eq!(tokens[1].lexeme, "سن");
        assert_eq!(
            tokens
                .iter()
                .filter(|token| token.kind == TokenKind::Identifier)
                .count(),
            2
        );
    }

    #[test]
    fn decodes_string_escapes() {
        let tokens = tokenize(r#"print("hello\nworld");"#).unwrap();
        assert_eq!(tokens[2].lexeme, "hello\nworld");
    }
}
