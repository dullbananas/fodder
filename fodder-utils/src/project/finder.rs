//abandoned file
use reqwest::{
    Client,
};
use std::{
    path::PathBuf,
    collections::HashMap,
};
use super::{
    ElmJson, Package, Application,
    Version, Constraint,
};


pub struct Installer {
    package_dir: PathBuf,
    registry: HashMap<String, Vec<Version>>,
}


impl Installer {
    pub fn new() -> Installer {
        // TODO: handle ELM_HOME environment variable
        let mut package_dir = dirs_next
            ::home_dir()
            .unwrap();
        package_dir
            .push(".elm/0.19.1/packages");
        
        // TODO: cache registry
        let registry = HashMap::<String, Vec<Version>>
            ::with_capacity(1024);

        Installer {
            package_dir,
            registry,
        }
    }

    pub async fn install_deps(&self, app: &Application) -> crate::Result<()> {
        let mut client = Client::new();
        let depStream
    }
}
