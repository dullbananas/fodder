#![deny(bare_trait_objects)]
#![deny(unused_must_use)]

// This module is accessed by macro-generated code
mod m {
    // Used to "rename" e.g. Thing to Visitor<Thing>
    pub struct Visitor<T>(
        pub ::std::marker::PhantomData<T>,
    );
}

#[macro_use] mod macros;

mod ast;
pub mod compiler;
mod error;
mod parse;
mod project;

use error::*;
