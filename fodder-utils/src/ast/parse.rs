use logos::{
    Lexer,
    Logos,
};
use super::{
    Token,
};


pub trait Parse: Sized {
    fn parse<'a>(buffer: &'a ParseBuffer<'a>) -> ParseResult<Self>;
}


pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
}


pub struct ParseBuffer<'a> {
    lexer: Lexer<'a, Token>,
}


impl<'a> ParseBuffer<'a> {
    pub fn new(src: &'a str) -> ParseBuffer<'a> {
        let mut lexer = Token::lexer(src);
        for t in &mut lexer {
            println!("{:?}", t);
        }
        ParseBuffer {
            lexer: lexer,
        }
    }


    // might use tokio::sync::mpsc in the future
    #[inline(always)]
    pub async fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }


    #[inline(always)]
    pub fn parse<T: Parse>(&'a self) -> ParseResult<T> {
        T::parse(self)
    }
}
