pub mod syntax;


use std::{
    fmt,
    io,
    panic::PanicInfo,
};
use serde_json::error as json;


pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug)]
pub enum Error {
    WrongPkgHash(String),
    ElmJsonParse {
        content: String,
        example: &'static str,
    },

    Io(io::Error),
    Json(json::Error),
    Reqwest(reqwest::Error),

    Syntax
}


impl Error {
    pub async fn report(&self) {
        println!("{:?}", self);
    }

    
    #[cold]
    pub fn report_panic(info: &PanicInfo) {
        println!("{title}{info}",
            title=Title("Error"),
            info=info,
        );
    }
}


struct Title(&'static str);

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-- {} ----\n\n", self.0)
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}
