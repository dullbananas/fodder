#![deny(unused_must_use)]

// This module is accessed by macros
mod m {
    // Used to "rename" e.g. Thing to Visitor<Thing>
    pub struct Visitor<T>(
        pub ::std::marker::PhantomData<T>,
    );
}

#[macro_use] pub(crate) mod macros;

pub mod project;
mod err;
pub mod ast;
pub mod compiler;

pub use err::{Result, Error};
