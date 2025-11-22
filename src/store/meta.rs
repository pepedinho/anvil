use std::{collections::HashMap, path::PathBuf, process::Command};

use anyhow::Result;
use clap::builder::Str;

pub enum ArtefactType {
    Bin,
    Int,
    Pack,
}

pub struct Dependency {
    name: String,
    version: String,
    hash: String,
}

pub struct Meta {
    artefact_hash: String,
    artefact_type: ArtefactType,
    created_at: String,

    git_commit: String,
    git_tree_hash: String,
    version: Option<String>,

    build_inputs: Vec<Dependency>,
    build_command: String,
    config_hash: String,

    prev_block_hash: Option<String>,
    block_hash: String,

    entrypoint: String,
    env: HashMap<String, String>,
}

pub fn get_last_commit() -> Result<String> {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get last_commit"));
    }

    let commit = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(commit)
}

fn build_project(build_command: &str) -> Result<PathBuf> {
    let status = Command::new("sh").arg("-c").arg(build_command).status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Build failed"));
    }

    Ok(PathBuf::from("target/release/myapp"))
}
