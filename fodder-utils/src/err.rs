use std::{
    fmt,
    io,
    panic::PanicInfo,
};
use serde_json::error as json;


pub type Result<T, E = Error> =
    std::result::Result<T, E>;


pub type MultiResult<T, E = Error> =
    std::result::Result<T, Vec<E>>;


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
}


impl Error {
    pub async fn report(&self) {
        println!("{:?}", self);
    }

    
    pub fn report_panic(info: &PanicInfo) {
        let payload = format_args!("");
        // i had some issues here with lifetimes
        /*let payload: fmt::Arguments = {
            // info.payload is Any. Usually it is a &str that explains the panic, but it can be any type. If it's a &str, opt_payload is Some. It will be None when it's any other type.
            let opt_payload: Option<&'static String> = info
                .payload()
                .downcast_ref::<String>();
            if let Some(&p) = opt_payload {
                format_args!("Message: {msg}\n",
                    msg=p,
                )
            } else {
                format_args!("")
            }
        };*/
        print!("{title}{info:?}\n{msg}",
            title=TITLE_PANIC,
            info=info,
            msg=payload,
        );
    }
}


struct Title(&'static str);


impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-- {} ------------\n\n", self.0)
    }
}


macro_rules! titles {
    // No lines left
    {} => {};
    {
        // First line
        $first_n:ident = $first_s:literal;
        // Other lines
        $(other_ns:ident = $other_ss:literal;)*
    } => {
        static $first_n: Title = Title($first_s);
        // Process other lines
        titles! {$($other_ns = $other_ss;)*}
    }
}


titles! {
    TITLE_PANIC = "Error";
}


// Generates impls for converting io::Error, etc. to Error (implicitly done when using the ? operator)
macro_rules! error_from {
    // No lines left
    {} => {};
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
