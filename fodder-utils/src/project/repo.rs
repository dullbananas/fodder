#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Repo(String, String);


impl Repo {
    pub fn author_project() -> Repo {
        Repo(
            "author".to_string(),
            "project".to_string(),
        )
    }
}


to_str! { Repo |Repo(a, b), f| {
    write!(f, "{}/{}", a, b)
}}


from_str! { Repo |rstr| {
    let parts: Vec<&str> = rstr
        .split('/')
        .collect();
    match parts.as_slice() {
        [a, b] => Ok(Repo(
            a.to_string(),
            b.to_string(),
        )),
        _ => Err(crate::Error::ElmJsonParse {
            content: rstr.to_string(),
            example: "elm/json",
        }),
    }
}}
