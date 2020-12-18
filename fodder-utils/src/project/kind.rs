#[derive(Debug)]
pub struct Application;
#[derive(Debug)]
pub struct Package;


from_str! { Application |kstr| {
    if kstr == "application" {
        Ok(Application)
    } else {
        Err(crate::Error::ElmJsonParse {
            content: kstr.to_string(),
            example: "application",
        })
    }
}}
from_str! { Package |kstr| {
    if kstr == "package" {
        Ok(Package)
    } else {
        Err(crate::Error::ElmJsonParse {
            content: kstr.to_string(),
            example: "package",
        })
    }
}}


to_str! { Application |_, f| {
    write!(f, "application")
}}
to_str! { Package |_, f| {
    write!(f, "package")
}}
