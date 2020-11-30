use serde::{Serialize, Deserialize};
use std::{
    path::PathBuf,
    collections::HashMap,
};
use tokio::{
    stream::StreamExt,
};
use tokio_compat_02::FutureExt;


type DepList<T> =
    HashMap<String, T>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all="kebab-case")]
#[serde(tag = "type")]
pub enum ElmJson {
    Application(Application),
    Package(Package),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct Application {
    source_directories: Vec<PathBuf>,
    elm_version: super::Version,
    dependencies: ApplicationDeps,
    test_dependencies: ApplicationDeps,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct Package {
    name: String,
    summary: String,
    license: super::License,
    version: super::Version,
    exposed_modules: Vec<crate::ast::name::Module>,
    elm_version: super::Constraint,
    dependencies: DepList<super::Constraint>,
    test_dependencies: DepList<super::Constraint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct ApplicationDeps {
    direct: DepList<super::Version>,
    indirect: DepList<super::Version>,
}


impl Application {
    pub async fn install_dependencies(&self, dir: &PathBuf, client: reqwest::Client) -> crate::Result<()> {
        let iter = std::iter::empty()
            .chain(self.dependencies.direct.iter())
            .chain(self.dependencies.direct.iter());
        let mut stream = tokio::stream
            ::iter(iter);
        while
            let Some((pkg_name, version)) = stream
                .next()
                .await
        {
            let mut pkg_dir = dir
                .clone();
            pkg_dir
                .push(format!(
                    "{}/{}", pkg_name, version
                ));
            tokio::fs::DirBuilder
                ::new()
                .recursive(true)
                .create(&pkg_dir)
                .await?;
            version
                .install(&pkg_dir, pkg_name, &client)
                .compat().await?;
        }
        Ok(())
    }
}


impl super::FromReader for ElmJson {
    const CAPACITY: usize = 512;
}
