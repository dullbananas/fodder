use crate::ast;
use std::future::Future;


pub struct State {
    pub ctx_stack: Vec<Ctx>,
    pub errors: Vec<ErrorInfo>,
    pub indent: u8,
    pub position: ast::Position,
}


pub struct ErrorInfo {
    kind: Error,
    position: ast::Position,
}


pub enum Error {
}


pub enum Ctx {
}


/// Controller
pub struct Co {
}/* final cursor position */
pub type PResult<'a, T, F> = Result<Resolve<'a, T>, Err<F>>;


pub fn byte<Fut>(byte: u8) -> impl Fn(&mut Co) -> Fut
where
    Fut: Future<Output = ()>,
{
    debug_assert_ne!(byte, b'\n',
        "Using \\n will make state.position wrong"
    );
    move |co| async {}
}



/*pub enum PositionChange {
    KeepLine {
        add_col: u32,
    },
    NewLine {
        add_row: u32,
        col: u32,
    },
}



pub struct Resolve<'a, T> {
    left_over: &'a [u8],
    pos: PositionChange,
    value: T,
}

pub enum Err<F> {
    Error(Error),
    Incomplete(Error, F),
}


pub trait Parser<'a, T, F>
where
    F: Fn(&'a [u8]) -> PResult<'a, T, F>,
{
    fn parse(
        self,
        state: &'a State,
        data: &'a [u8],
    ) -> PResult<'a, T, F>;
}


pub struct Byte {
    byte: u8,
    error: Error,
}

impl<'a, F> Parser<'a, (), F> for Byte
where
    F: Fn(&'a [u8]) -> PResult<'a, (), F>,
{
    fn parse(
        self,
        state: &'a State,
        data: &'a [u8],
    ) -> PResult<'a, (), F> {

        match data.split_first() {
            Some((x, xs))
            if *x == self.byte => Ok(
                Resolve {
                    left_over: xs,
                    pos: PositionChange::KeepLine {
                        add_col: 1,
                    },
                    value: (),
                }
            ),
            Some(_) => Err(
                Err::Error(
                    self.error,
                )
            ),
            None => Err(
                Err::Incomplete(
                    self.error,
                    |data| self.parse(state, data),
                )
            ),
        }
    }
}*/


/*pub enum PollParse<'a, T> {
    Pending,
    Ready {
        result: Option<(&'a [u8], T)>,
        state: State,
    },
    /// Should be unreachable
    EmptyDataError,
}
pub use PollParse::*;


pub trait Parser<'a, T> {
    type Running: RunningParser<'a, T>;
    fn new(self, state: &'a State) -> Self::Running;
}


pub trait RunningParser<'a, T> {
    fn poll_parse(&mut self, data: &'a [u8]) -> PollParse<'a, T>;
}


pub fn byte<'a>(byte: u8) -> impl Parser<'a, ()> {

    struct Byte(u8);
    impl<'a> Parser<'a, ()> for Byte {
        type Running = RunningByte<'a>;
        fn new(self, state: &'a State) -> Self::Running {
            RunningByte(state, self.0)
        }
    }
    
    struct RunningByte<'a>(&'a State, u8);
    impl<'a> RunningParser<'a, ()> for RunningByte<'a> {
        fn poll_parse(&mut self, data: &'a [u8]) -> PollParse<'a, ()> {
            match data.split_first() {
                None =>
                    EmptyDataError,
                Some((x, xs)) if *x == self.1 =>
                    Ready {
                        result: Some((xs, ())),
                        state: State {
                            position: self.0.position.next_col(),
                            ..*self.0
                        }
                    }
            }
        }
    }

    Byte(byte)
}*/
