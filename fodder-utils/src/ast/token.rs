use logos::{
    Filter,
    Logos,
};


type Lexer<'a, T = Token> = logos::Lexer<'a, T>;


// use (?x) at the beginning of regex patterns to ignore whitespace
#[derive(Logos, Debug)]
pub enum Token {
    // Line break + indent; ignores blank lines
    #[regex(r"\n[ \n]*", newline)]
    Newline(usize),

    #[token("alias")]
    KwAlias,
    #[token("as")]
    KwAs,
    #[token("case")]
    KwCase,
    #[token("effect")]
    KwEffect,
    #[token("else")]
    KwElse,
    #[token("exposing")]
    KwExposing,
    #[token("if")]
    KwIf,
    #[token("import")]
    KwImport,
    #[token("in")]
    KwIn,
    #[token("let")]
    KwLet,
    #[token("module")]
    KwModule,
    #[token("of")]
    KwOf,
    #[token("port")]
    KwPort,
    #[token("then")]
    KwThen,
    #[token("type")]
    KwType,
    #[token("where")]
    KwWhere,

    #[token("->")]
    Arrow,
    #[token("\\")]
    Backslash,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("..")]
    Ellipsis,
    #[token("=")]
    Eq,
    #[token("|")]
    Pipe,

    #[regex(r"(?x) [
        |
        >
        <
        \+
        \-
        /
        \*
        \.
        =
        :
        ^
        &
        ?
        %
        !
    ]+", string)]
    Infix(String),

    #[token("{")]
    LBrace,
    #[token("[")]
    LBracket,
    #[token("(")]
    LParen,
    #[token("}")]
    RBrace,
    #[token("]")]
    RBracket,
    #[token(")")]
    RParen,

    // TODO: support fancy characters like accents
    #[regex(r"(?x)
        ([A-Z][a-zA-Z0-9_]*\.)*
        ([a-z][a-zA-Z0-9_]*\.)*
        ([a-z][a-zA-Z0-9_]*)
    ", path)]
    PathLowercase(Vec<String>),
    #[regex(r"(?x)
        ([A-Z][a-zA-Z0-9_]*\.)*
        ([A-Z][a-zA-Z0-9_]*)
    ", path)]
    PathUppercase(Vec<String>),
    
    #[regex(r"\.[a-z][a-zA-Z0-9_]*", accessor)]
    RecordAccessor(String),

    // TODO
    #[regex("'.'")]
    LitChar,
    #[regex(r"[0-9]+\.[0-9]+", float)]
    LitFloat(f64),
    #[regex(r"[0-9]+", int)]
    LitInt(i32),
    // TODO: support unicode escape (e.g. `\u{FFFFF}`)
    #[regex(r#"(?x)"
        ( [^\n"\\]
        | (\\[n"t'\\r])
        )*
    ""#, (str_quote)(1))]
    LitString(String),
    // TODO: multiline strings

    #[regex(r"\t+")]
    ErrorTab,
    // Multi line nestable comment
    #[regex(r"[\n ]*\{-+[^\{\}-]*", comment)]
    ErrorEndlessComment,
    #[error]
    // Spaces
    #[regex(r"[ ]+", logos::skip)]
    // Single line comment
    #[regex(r"[\n ]*--[^\n]*\n?", logos::skip)]
    Error,
}


fn str_quote(qlen: usize) -> impl FnMut(&mut Lexer) -> String {
    #[derive(Logos)]
    enum Str {
        #[regex(r"\\.", (nth_char)(1))]
        Escape(char),
        #[regex(r"[^\\]", (nth_char)(0))]
        Char(char),
        #[error]
        Error,
    }

    fn nth_char(n: usize) -> impl FnMut(&mut Lexer<Str>) -> char {
        move |lex| {
            let c = lex
                .slice()
                .chars()
                .nth(n);
            match c {
                Some(jc) => jc,
                None => unreachable!(),
            }
        }
    }

    fn escape(char: char) -> char {
        match char {
            'n' => '\n',
            '"' => '"',
            't' => '\t',
            '\'' => '\'',
            '\\' => '\\',
            'r' => '\r',
            // prevented by regex
            _ => unreachable!(),
        }
    }

    move |main_lexer| {
        let len = {
            let s = main_lexer.span();
            s.end - s.start
        };
        let mut result = String
            ::with_capacity(len);
        
        let content = main_lexer
            .slice()
            .get(qlen..len-qlen)
            .unwrap();

        for token in Str::lexer(content) {
            result.push(match token {
                Str::Char(c) => c,
                Str::Escape(c) => escape(c),
                // prevented by Token regex
                _ => unreachable!(),
            });
        }

        result
    }
}




fn comment(main_lexer: &mut Lexer) -> Filter<()> {
    #[derive(Logos, Debug, Copy, Clone)]
    enum Comment {
        #[regex(r"\{-+[^\{\}-]*")]
        Left,
        #[regex("-+}")]
        Right,
        #[error]
        // `{` not followed by `-`
        #[regex(r"\{[^-][^\{\}-]*", logos::skip)]
        // `-` not followed by `}`
        #[regex(r"-[^\}][^\{\}-]*", logos::skip)]
        // `\n`
        #[regex(r"\n[^\{\}-]*", logos::skip)]
        Error,
    }
    let mut lexer = Comment
        ::lexer(main_lexer.remainder());
    let mut depth: i8 = 1;
    for token in &mut lexer {
        depth += match token {
            Comment::Right => -1,
            Comment::Left => 1,
            Comment::Error => continue,
        };
        if depth == 0 {
            main_lexer
                .bump(lexer.span().end);
            //println!("{}", main_lexer.slice());
            return Filter::Skip;
        }
    }
    Filter::Emit(())
}


fn accessor(lexer: &mut Lexer) -> String {
    lexer
        .slice()[1..]
        .to_string()
}


fn path(lexer: &mut Lexer) -> Vec<String> {
    lexer
        .slice()
        .split('.')
        .map(String::from)
        .collect()
}


fn int(lexer: &mut Lexer) -> Option<i32> {
    match lexer.slice().parse() {
        Ok(n) => Some(n),
        // TODO: use the IntErrorKind (when it becomes stable) in a `match` and make certain errors unreachable if the regex prevents it
        Err(_) => None,
    }
}


fn float(lexer: &mut Lexer) -> Option<f64> {
    match lexer.slice().parse() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}


fn string(lexer: &mut Lexer) -> String {
    lexer.slice().to_string()
}


fn newline(lexer: &mut Lexer) -> usize {
    let mut indent = 0;
    let mut chars = lexer
        .slice()
        .chars();
    // Ignore initial newline
    let _ = chars.next();
    for char in chars {
        indent = match char {
            ' ' => indent + 1,
            '\n' => 0,
            _ => unreachable!(),
        };
    }
    indent
}
