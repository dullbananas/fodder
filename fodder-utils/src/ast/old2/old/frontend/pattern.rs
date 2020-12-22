use super::{name, Tuple};


pub enum Pattern {
    Underscore,
    Var(name::Var),
    Enum {
        variant: name::Path<name::EnumVariant>,
        args: Vec<Box<Pattern>>,
    },
    Tuple(Tuple<Pattern>),
}
