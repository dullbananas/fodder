use super::{Expr, Pattern, Span};


pub struct Binding {
    pattern: Pattern,
    body: Span<Expr>,
}
