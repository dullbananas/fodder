use super::{Span};


pub struct Module(Vec<String>);


pub struct EnumVariant(Span<String>);
pub struct Type(Span<String>);
pub struct Var(Span<String>);


pub struct Path<T> {
    module: Option<Module>,
    item: T,
}
