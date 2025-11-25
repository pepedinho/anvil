use std::{process::Command, time::SystemTime};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ArtefactType {
    Bin,
    Int,
    Pack,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {
    name: String,
    version: String,
    hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub artefact_hash: String,
    pub artefact_type: ArtefactType,
    pub created_at: SystemTime,
    pub version: String,

    pub git_commit: String,
    // pub git_tree_hash: String,
    // pub version: Option<String>,

    // pub build_inputs: Vec<Dependency>,
    // pub build_command: String,
    // pub config_hash: String,
    pub prev_block_hash: Option<String>,
    pub block_hash: String,
    pub entrypoint: String,
    // pub env: HashMap<String, String>,
}

pub fn get_last_commit() -> Result<String> {
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get last_commit"));
    }

    let commit = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(commit)
}
