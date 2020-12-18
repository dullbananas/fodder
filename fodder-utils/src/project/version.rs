use std::{
    num::ParseIntError,
    path::PathBuf,
    io::Cursor,
};
use reqwest::Client;
use serde::Deserialize;
use sha1::{Sha1, Digest};
use super::{Repo};


#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Version(pub u8, pub u8, pub u8);

from_str! { Version |vstr| {
    let nums: Vec<Result<u8, ParseIntError>> = vstr
        .split('.')
        .map(|s| u8::from_str_radix(s, 10))
        .collect();
    match nums.as_slice() {
        [Ok(a), Ok(b), Ok(c)] => Ok(
            Version(*a, *b, *c)
        ),
        _ => Err(crate::Error::ElmJsonParse {
            content: vstr.to_string(),
            example: "1.0.0",
        }),
    }
}}

to_str! { Version |Version(a, b, c), f| {
    write!(f, "{}.{}.{}", a, b, c)
}}


#[derive(Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
struct EndpointJson {
    url: String,
    #[serde(with = "hex")]
    hash: [u8; 20],
}


impl Version {
    // Assumes the package is not already installed
    // Doesn't install package's dependencies
    pub async fn install(&self, dir: &PathBuf, pkg_name: &Repo, client: &Client) -> crate::Result<()> {
        let endpoint_json = {
            let url = format!(
                "https://package.elm-lang.org/packages/{}/{}/endpoint.json", pkg_name, self
            );
            let response = client
                .get(&url)
                .send()
                .await?;
            response
                .json::<EndpointJson>()
                .await?
        };
        let check_hash = |correct_hash, data: &Vec<u8>| {
            let hash: [u8; 20] = Sha1
                ::digest(&data[..])
                .into();
            if correct_hash != hash {
                return Err(
                    crate::Error::WrongPkgHash(
                        pkg_name.to_string()
                    )
                );
            }
            Ok(())
        };
        let zip_reader: Cursor<Vec<u8>> = {
            let response = client.get(&endpoint_json.url).send().await?;
            let bytes: Vec<u8> = response
                .bytes().await?[..]
                .into();
            check_hash(endpoint_json.hash, &bytes)?;
            Cursor::new(bytes)
        };
        let strip_top_dir = true;
        zip_extract
            ::extract(zip_reader, &dir, strip_top_dir)
            .unwrap();
        Ok(())
    }
}
