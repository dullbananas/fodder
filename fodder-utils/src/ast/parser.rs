use tree_sitter::{self as ts, Tree};
use std::collections::HashMap;
use tokio::{
    fs::File,
    prelude::*,
};
use super::{ModuleId, ParseBuffer};


pub struct Parser {
    ts_parser: ts::Parser,
    errors: Vec<crate::Error>,
}


impl Parser {
    pub fn new() -> Parser {
        Parser {
            ts_parser: {
                let mut p = ts::Parser::new();
                let lang = tree_sitter_elm::language();
                p.set_language(lang).unwrap();
                p.set_logger(Some(Box::new(|kind, msg| {
                    println!("{:?}: {}", kind, msg);
                })));
                p
            },
            errors: Vec::with_capacity(16),
        }
    }


    pub async fn add_module(&mut self, id: ModuleId) -> crate::Result<()> {
        let lines: Vec<String>;
        let tree: Tree = {
            let mut buf = Vec
                ::with_capacity(1024);
            let mut file = File
                ::open(&id.path)
                .await?;
            file.read_to_end(&mut buf)
                .await?;
            // Create the value of `lines` in the background while `tree` is created
            let buf2 = buf.clone();
            let lines_task = tokio
                ::spawn(async move {
                    String
                        ::from_utf8(buf2.clone())
                        // Ok if file is valid UTF-8
                        .unwrap()
                        .lines()
                        .map(String::from)
                        .collect::<Vec<String>>()
                });
            let tree = self.ts_parser
                .parse(buf, None)
                .unwrap();
            lines = lines_task
                .await
                .unwrap();
            tree
        };

        let cursor = tree
            .walk();

        let stream = ParseBuffer
            ::new(&id, lines, cursor)
            .await?;
        
        let mut module_streams = HashMap
            ::<ModuleId, ParseBuffer>
            ::with_capacity(64);

        module_streams.insert(
            id,
            stream,
        );
        
        Ok(())
    }
}
