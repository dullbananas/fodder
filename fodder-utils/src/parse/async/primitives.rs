// An async implementation which is currently unused but might be used in the feature.


use crate::ast;
use std::task::Poll;
use tokio::io::AsyncReadExt;


pub struct State {
    indent: u16,
    position: ast::Position,
}

impl State {
    fn start() -> State {
        State {
            indent: 0,
            position: ast::Position {
                row: 1,
                col: 1,
                offset: 0,
            },
        }
    }
}


pub trait Builder<'a, T, E> {
    type Out;
    fn ok_consumed(
        //&self,
        value: T,
        state: impl Fn(&'a mut State),
    ) -> Self::Out;
    fn ok_empty(
        //&self,
        value: T,
        state: impl Fn(&'a mut State),
    ) -> Self::Out;
    fn err_consumed(
        //&self,
        err: impl Fn(ast::Position) -> E,
    ) -> Self::Out;
    fn err_empty(
        //&self,
        err: impl Fn(ast::Position) -> E,
    ) -> Self::Out;
}


pub enum PResult<'a, P, B>
where
    P: Parser<'a, B>,
    B: Builder<'a, P::Value, P::Error>,
{
    Continue(&'a mut P),
    Finish(B::Out),
}
pub use PResult::*;


pub trait Parser<'a, B: Builder<'a, Self::Value, Self::Error>>: Sized {
    //type Builder: Builder<'a, Self::Value, Self::Error>;
    type Error;
    type Value;
    fn start(state: &State, builder: &B) -> Self;
    fn feed(&mut self, bytes: &'a [u8]) -> PResult<'a, Self, B>;
}


/*pub struct RootBuilder<F, E>
where
    F: Fn(ast::Position) -> E,
{
    badEnd: F,
}

impl RootBuilder {
    #[inline]
    fn to_ok<T, E>(
        &self,
        value: T,
        state: impl Fn(&mut State),
    ) -> Result<T, E> {
        
    }
}

impl<'a, T, E> Builder<'a, T, E> for RootBuilder {
    type Out = Result<T, E>;

    fn val
}*/


// Very old code below


/*pub struct State {
    pub bytes: Vec<u8>,
    pub ctx_stack: Vec<Ctx>,
    pub errors: Vec<ErrorInfo>,
    pub file: tokio::fs::File,
    pub indent: u8,
    pub position: ast::Position,
}


impl State {
    fn add_error<E: PushError>(&mut self, error: E) -> Option<()> {
        error.push_to(self);
        E::RETURN
    }

    pub async fn read<E: PushError>(&mut self, eof_error: E) -> Option<usize> {
        let res = self.file
            .read_buf(&mut self.bytes)
            .await;
        match res {
            Ok(0) => self.add_error(
                eof_error
            )?,
            Ok(n) => {
                assert!(self.bytes.len() > self.position.offset);
                return Some(dbg!(n));
            },
            Err(e) => self.add_error(
                Error::Io(e.kind())
            )?,
        }
        // Only reachable if `eof_error` is `()`
        Some(0)
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes.len() - self.position.offset
    }
}


pub struct ErrorInfo {
    kind: Error,
    position: ast::Position,
}


#[derive(Copy, Clone)]
pub enum Error {
    Io(std::io::ErrorKind),
}


/// `()` or `Error`
pub trait PushError: Copy {
    /// Used for `?` chaining
    const RETURN: Option<()>;
    fn push_to(self, state: &mut State);
}

impl PushError for () {
    const RETURN: Option<()> = Some(());
    fn push_to(self, _state: &mut State) {}
}

impl PushError for Error {
    const RETURN: Option<()> = None;
    fn push_to(self, state: &mut State) {
        state.errors.push(ErrorInfo {
            kind: self,
            position: state.position,
        });
    }
}


pub enum Ctx {
}


pub struct Byte<E: PushError>(pub u8, pub E);


impl<E: PushError> Byte<E> {
    pub async fn expect(&'static self, s: &mut State) -> Option<()> {
        let Byte(byte, error) = *self;
        assert_ne!(byte, b'\n',
            "Using \\n will make state.position wrong after parsing"
        );
        if s.remaining_bytes() == 0 {
            s.read(error).await?;
        }
        if s.bytes[s.position.offset] == byte {
            s.position.offset += 1;
            s.position.col += 1;
            return Some(());
        }
        s.add_error(error);
        None
    }
}


pub struct Bytes<E: PushError>(pub &'static [u8], pub E);


impl<E: PushError> Bytes<E> {
    pub async fn attempt(&'static self, s: &mut State) -> Option<()> {
        let Bytes(bytes, error) = *self;
        assert!(
            !bytes.contains(&b'\n'),
            "Using \\n will make state.position wrong after parsing",
        );
        // A slice of the expected bytes
        let mut remaining = &bytes[..];
        // As `remaining` shrinks, this increases
        let mut offset: usize = s.position.offset;
        loop {
            let available = s.bytes.len() - offset;
            if s.bytes[offset..] != remaining[..available] {
                s.add_error(error);
                return None;
            }
            offset += available;
            remaining = &remaining[available..];
            if remaining.len() == 0 {
                break;
            }
            s.read(error).await?;
        }
        // Expect whitespace 
        s.position.offset += offset;
        s.position.col += offset;
        Some(())
    }
}*/
