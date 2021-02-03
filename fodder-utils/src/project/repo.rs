#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Repo {
    user: String,
    name: String,
}


impl Repo {
    /*pub fn author_project() -> Repo {
        Repo(
            "author".to_string(),
            "project".to_string(),
        )
    }*/
}


to_str! { Repo |Repo {user, name}, f| {
    write!(f, "{}/{}", user, name)
}}


from_str! { Repo |rstr| {
    let parts: Vec<&str> = rstr
        .split('/')
        .collect();
    match parts.as_slice() {
        [user, name] => Ok(Repo {
            user: user.to_string(),
            name: name.to_string(),
        }),
        _ => Err(crate::Error::ElmJsonParse {
            content: rstr.to_string(),
            example: "elm/json",
        }),
    }
}}
