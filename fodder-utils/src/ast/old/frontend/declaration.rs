use super::{Expr, name, Span, TypeAnnotation};


pub enum Declaration {
    Value {
        name: name::Var,
        type_annotation: Option<Span<TypeAnnotation>>,
        expr: Expr,
    },
    TypeAlias {
        name: Span<name::Type>,
        params: Vec<name::Var>,
        def: Span<TypeAnnotation>,
    },
    Enum {
        name: name::Type,
        params: Vec<name::Var>,
        def: Vec<Span<EnumVariant>>,
    },
}


pub struct EnumVariant {
    name: name::EnumVariant,
    args: Vec<Span<TypeAnnotation>>,
}
