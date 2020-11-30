#[derive(Debug)]
pub struct License(String);

to_str! { License
    |License(s), f| {
        write!(f, "{}", s)
    }
}

from_str! { License
    |lstr| {
        match spdx::license_id(lstr) {
            Some(_) => Some(License(lstr.to_string())),
            None => None,
        }
    }
}