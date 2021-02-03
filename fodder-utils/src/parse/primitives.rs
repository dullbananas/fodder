use crate::ast;
use nom::{
    IResult,
};


/*pub struct State<'src> {
    indent: u16,
    position: ast::Position,
    source_bytes: &'src [u8],
}


#[derive(Clone)]
pub struct Builder<'a, 'src, T, E, Out> {
    ok_consumed: &'a dyn Fn(T, State<'src>) -> Out,
    ok_empty: &'a dyn Fn(T, State<'src>) -> Out,
    err_consumed: &'a dyn Fn(E) -> Out,
    err_empty: &'a dyn Fn(E) -> Out,
}


pub trait Parser<Out> {
    type Error;
    type Value;
    fn parse<'a, 'src>(
        &self,
        state: &'src State<'src>,
        builder: &'a Builder<'a, 'src, Self::Value, Self::Error, Out>,
    ) -> Out;
}


pub struct OneOf<'err, 'p, T, E, Out> {
    pub error: &'err dyn Fn(ast::Position) -> E,
    pub parsers: &'p [Box<dyn Parser<Out, Error = E, Value = T>>],
}


impl<'err, 'p, T, E, Out> Parser<Out> for OneOf<'err, 'p, T, E, Out> {
    type Error = E;
    type Value = T;
    fn parse<'a, 'src>(
        &self,
        state: &'src State<'src>,
        builder: &'a Builder<'a, 'src, T, E, Out>,
    ) -> Out {
        match self.parsers.split_first() {
            Some((
                // Two lines because rust-anal shows long type annotations here
                parser,
                parsers,
            )) => {
                let err_empty = |_err| {
                    OneOf {
                        error: self.error,
                        parsers: parsers,
                    }.parse(state, builder)
                };
                parser.parse(state, &Builder {
                    err_empty: &err_empty,
                    ..*builder
                })
            },

            None => {
                let error = (self.error)(state.position.clone());
                (builder.err_empty)(error)
            }
        }
    }
}*/
