use crate::project::ModuleId;
use super::{
    ParseBuffer,
    ParseError,
};
use tokio::{
    fs::File,
    prelude::*,
};


pub struct Parser {
    errors: Vec<ParseError>,
}


impl Parser {
    pub fn new() -> Parser {
        Parser {
            errors: Vec::with_capacity(16),
        }
    }

    pub async fn add_module(&mut self, id: ModuleId) -> crate::Result<()> {
        let mut buf = Vec
            ::with_capacity(1024);
        let mut file = File
            ::open(&id.path)
            .await?;
        file
            .read_to_end(&mut buf)
            .await?;
        let str = String
            ::from_utf8(buf)
            // Ok if file is valid UTF-8
            .unwrap();
        let result = tokio::spawn(async move {
            let buf = ParseBuffer
                ::new(str.as_ref());
        }).await;
        // Err if a panic occurs in the task. This only unwraps the Result that has task::JoinError.
        let result = match result {
            Ok(r) => r,
            Err(e) => std::panic
                ::resume_unwind(e.into_panic()),
        };
        Ok(())
    }
}
