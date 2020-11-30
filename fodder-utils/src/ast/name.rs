#[derive(Debug)]
pub struct Module(Vec<String>);


// for exposed-modules in elm.ElmJson
from_str! { Module |mstr| {
    let parts = mstr
        .split('.')
        .map(str::to_string)
        .collect();
    Some(Module(parts))
}}

to_str! { Module |Module(parts), f| {
    write!(f, "{}", parts.join("."))
}}
