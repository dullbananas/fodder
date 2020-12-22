use super::Repo;
use std::path::PathBuf;


#[derive(Hash, Eq, PartialEq)]
pub struct ModuleId {
    pub repo: Repo,
    pub name: Vec<String>,
    pub path: PathBuf,
}
