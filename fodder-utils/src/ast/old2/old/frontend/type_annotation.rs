use super::{name, Span, Tuple};


pub enum TypeAnnotation {
    Var(name::Var),
    Fn(Span<TypeAnnotation>, Span<TypeAnnotation>),
    List(Span<TypeAnnotation>),
    Tuple(Tuple<Span<TypeAnnotation>>),
    Name {
        path: name::Path<name::Type>,
        args: Vec<Span<TypeAnnotation>>,
    },
}
