use super::{Declaration, Exposing, Import, name};
use std::collections::HashMap;


pub struct Module {
    pub kind: Kind,
    pub name: name::Module,
    pub exposing: Exposing,
    pub imports: Vec<Import>,
    pub declarations: HashMap<name::Var, Declaration>,
}


pub enum Kind {
    Plain,
    Port,
    Effect {
        command: Option<name::Type>,
        subscription: Option<name::Type>,
    },
}
