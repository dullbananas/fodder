use super::{Exposing, name};


pub struct Import {
    pub name: name::Module,
    pub alias: Option<name::Module>,
    pub exposing: Option<Exposing>,
}
