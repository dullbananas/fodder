use serde::{Serialize, Deserialize};
use std::{
    path::PathBuf,
    collections::HashMap,
};
use tokio::{
    fs::File,
    prelude::*,
};
use tokio_compat_02::FutureExt;
use super::{Constraint, kind, License, Repo, Version};
use crate::ast;


type DepList<T> =
    HashMap<Repo, T>;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct Application {
    #[serde(rename = "type")]
    kind: kind::Application,
    source_directories: Vec<PathBuf>,
    elm_version: Version,
    dependencies: ApplicationDeps,
    test_dependencies: ApplicationDeps,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct Package {
    #[serde(rename = "type")]
    kind: kind::Package,
    name: Repo,
    summary: String,
    license: License,
    version: Version,
    exposed_modules: Vec<String>,
    elm_version: Constraint,
    dependencies: DepList<Constraint>,
    test_dependencies: DepList<Constraint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub struct ApplicationDeps {
    direct: DepList<Version>,
    indirect: DepList<Version>,
}


impl Application {
    pub fn iter_dependencies<'a>(&'a self) -> impl Iterator<Item = (&'a Repo, &'a Version)> + 'a {
        let direct = self.dependencies.direct
            .iter();
        let indirect = self.dependencies.indirect
            .iter();
        direct.chain(indirect)
    }


    pub async fn install_dependencies(&self, dir: &PathBuf, client: reqwest::Client) -> crate::Result<()> {
        for (pkg_name, version) in self.iter_dependencies() {
            let mut pkg_dir = dir.clone();
            pkg_dir.push(format!(
                "{}/{}", pkg_name, version
            ));
            tokio::fs::DirBuilder::new()
                .recursive(true)
                .create(&pkg_dir)
                .await?;
            version
                .install(&pkg_dir, pkg_name, &client)
                .compat().await?;
        }
        Ok(())
    }


    pub async fn from_path(path: &mut PathBuf) -> crate::Result<Self> {
        let mut result: Self = {
            let mut file = File::open(&*path)
                .await?;
            let mut bytes = Vec::<u8>
                ::with_capacity(1024);
            file.read_to_end(&mut bytes)
                .await?;
            serde_json::from_reader(&bytes[..])
        }?;

        path.pop();
        result.prepend_paths(path);
        Ok(dbg!(result))
    }


    fn prepend_paths(&mut self, prefix: &PathBuf) {
        for dir in self.source_directories.iter_mut() {
            let mut new_dir = prefix.clone();
            new_dir.push(dir.clone());
            *dir = new_dir;
        }
    }

    
    pub async fn create_ast(&self) -> crate::Result<()> {
        let mut parser = ast::Parser::new();
        parser.add_module( ast::ModuleId {
            repo: Repo::author_project(),
            name: vec!["Main".to_string()],
            path: {
                let mut path = self
                    .source_directories[0]
                    .clone();
                path.push("Main.elm");
                path
            },
        }).await?;
        Ok(())
    }
}
