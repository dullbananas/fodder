use tree_sitter::TreeCursor;


pub type ParseResult<T> =
    Result<T, ParseError>;


pub struct ParseError {
    kind: ParseErrorKind,
}


pub enum ParseErrorKind {
    Expect {
        expecting: &'static str,
    },
    Unknown,
}


pub trait Parse {
    // example: "a type"
    const DESCRIPTION: &'static str;

    fn parse<'a>(buffer: &'a ParseBuffer<'a>) -> ParseResult<Self>
    where
        Self: Sized;
}

pub struct ParseBuffer<'a> {
    src: Vec<String>,
    cursor: TreeCursor<'a>,
}


impl<'a> ParseBuffer<'a> {
    pub async fn new(
        lines: Vec<String>,
        cursor: TreeCursor<'a>,
    ) -> crate::Result<ParseBuffer<'a>>
    {
        let mut ms = Self {
            src: lines,
            cursor: cursor,
        };

        ms.dbg(0);
        {
            ms.enter_field("moduleDeclaration");
            // TODO: validate module name
            ms.goto_parent();
        }
        Ok(ms)
    }

    // TODO: error handling
    pub fn enter_field(&mut self, name: &'static str) {
        let id = self.cursor
            .node()
            .language()
            .field_id_for_name(name)
            .unwrap();
        if !self.cursor.goto_first_child() {
            panic!();
        }
        // do-while
        while {
            if self.cursor.field_id() == Some(id) {
                return;
            }
            self.cursor.goto_next_sibling()
        } {}
        panic!("missing field: {}", name);
    }

    pub fn goto_parent(&mut self) {
        if self.cursor.goto_parent() {
            ()
        } else {
            unreachable!()
        }
    }

    pub fn parse<T: Parse>(&self) -> ParseResult<T> {
        T::parse(self)
    }

    /// Formats and displays the current node.
    ///
    /// # Arguments
    ///
    /// * `level` - Set this to 0.
    pub fn dbg(&mut self, level: usize) {
        let indent = "|  "
            .repeat(level);

        // format_args not used here..
        let label = {
            self.cursor.field_name()
                // ..because s is dropped here
                .map(|s| format!("field {}", s))
        }.unwrap_or("_".to_string());

        // idk why format_args doesn't work here
        let kind = format!("{un}named {kind}",
            un =
                if self.cursor.node().is_named()
                {""}
                else {"un"},
            kind =
                self.cursor.node().kind(),
        );

        println!("{indent}{label}: {kind}",
            indent = indent,
            label = label,
            kind = kind,
        );

        // Display child nodes
        if self.cursor.goto_first_child() {
            let new_level = level + 1;
            // do-while
            while {
                self.dbg(new_level);
                self.cursor.goto_next_sibling()
            } {}
            self.goto_parent();
        }
    }
}
