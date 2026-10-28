use crate::source::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Array(Box<Type>),
}

impl Type {
    pub fn array(element: Type) -> Self {
        Self::Array(Box::new(element))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => f.write_str("int"),
            Self::Float => f.write_str("float"),
            Self::Bool => f.write_str("bool"),
            Self::String => f.write_str("string"),
            Self::Void => f.write_str("void"),
            Self::Array(element) => write!(f, "{element}[]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Literal(Literal),
    Array(Vec<Expr>),
    Identifier(String),
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    IndexAssignment {
        target: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub parameter_type: Type,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    VariableDeclaration {
        is_final: bool,
        variable_type: Type,
        name: String,
        initializer: Expr,
    },
    FunctionDeclaration {
        return_type: Type,
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
    ForEach {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
    Print {
        arguments: Vec<Expr>,
        separator: Option<Expr>,
        end: Option<Expr>,
    },
    Expression(Expr),
}

pub type Program = Vec<Stmt>;
