use super::{name};


pub enum Exposing {
    All,
    List(Vec<Item>),
}


pub enum Item {
    Var(name::Var),
    Type {
        name: name::Type,
        children: bool,
    },
}
