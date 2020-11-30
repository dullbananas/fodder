use std::{
    fmt,
    io,
};
use serde_json::error as json;
//use zip_extract::{ZipExtractError, ZipError};


pub type Result<T, E = Error> =
    std::result::Result<T, E>;


#[derive(Debug)]
pub enum Error {
    WrongPkgHash(String),

    Io(io::Error),
    Json(json::Error),
    Reqwest(reqwest::Error),
}


impl Error {
    pub async fn report(self) {
        Result::<()>::Err(self).unwrap();
    }
}


// Generates impls for converting io::Error, etc. to Error (implicitly done when using the ? operator)
macro_rules! error_from {
    {
        // First line
        $first_t:path, $first_v:path;
        // Other lines
        $($other_ts:path, $other_vs:path;)*
    } => {
        impl From<$first_t> for $crate::err::Error {
            fn from(err: $first_t) -> Self {
                $first_v(err)
            }
        }
        // Process other lines
        error_from! {$($other_ts, $other_vs;)*}
    };
    // No lines left
    {} => {};
}

error_from! {
    io::Error, Error::Io;
    json::Error, Error::Json;
    reqwest::Error, Error::Reqwest;
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}
