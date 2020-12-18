use super::{Binding, Case, name, Pattern, Span, Tuple};
use std::collections::BTreeMap;


pub enum Expr {
    Int(i32),
    Float(f64),
    Char(char),
    String(String),
    VarRef(name::Path<name::Var>),
    Lambda {
        // TODO: support argument unpacking
        args: Vec<Span<Pattern>>,
        body: Span<Expr>,
    },
    Call {
        func: Span<Expr>,
        arg: Span<Expr>,
    },
    If {
        condition: Span<Expr>,
        then_body: Span<Expr>,
        else_body: Box<Expr>,
    },
    Let {
        bindings: Vec<Span<Binding>>,
        body: Span<Expr>,
    },
    List(Vec<Span<Expr>>),
    Tuple(Tuple<Expr>),
    Case(Case<Pattern, Expr>),
    Record,
}
