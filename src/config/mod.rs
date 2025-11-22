use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

pub mod parse;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub project: Project,
    pub build: Build,
    pub dependency_script: Option<String>,
    // pub dependencies: Vec<Dependency>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Build {
    pub artifact_dir: PathBuf,
    pub entrypoint: PathBuf,
    pub command: String,
    pub incremental: bool,
    pub jit: bool,
}

// #[derive(Debug, Deserialize)]
// pub struct Dependency {
//     pub language: String,
//     pub package: Option<Vec<Package>>,
// }

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}
