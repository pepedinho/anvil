use std::{fs, path::PathBuf};

pub mod cmd;
pub mod tests;

use crate::{
    cli::{Cli, Commands},
    config::Config,
    store::{
        meta::{Meta, get_last_commit},
        traits::Store,
    },
};

#[derive(Debug)]
pub struct AnvilCore<S: Store> {
    pub config: Config,
    pub store: S,
    pub blocks: Vec<Meta>,
    pub current_commit: Option<String>,
    pub project_root: PathBuf,
}

impl<S: Store> AnvilCore<S> {
    pub fn new(config: Config, store: S, project_root: PathBuf) -> anyhow::Result<Self> {
        let anvil_dir = project_root.join(".anvil");
        if !anvil_dir.exists() {
            fs::create_dir_all(&anvil_dir)?;
        }

        let blocks_path = anvil_dir.join("blocks.json");
        let blocks = if blocks_path.exists() {
            let content = fs::read_to_string(&blocks_path)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };

        let last_commit = get_last_commit()?;

        Ok(Self {
            config,
            store,
            blocks,
            current_commit: Some(last_commit),
            project_root,
        })
    }

    pub fn interpret(&mut self, cli: &Cli) -> anyhow::Result<()> {
        match cli.command {
            Commands::Pack => self.pack(),
            Commands::Install { url: _ } => Ok(()),
            Commands::Switch {
                project: _,
                version: _,
            } => Ok(()),
        }
    }

    fn anvil_dir(&self) -> PathBuf {
        self.project_root.join(".anvil")
    }

    fn blocks_file(&self) -> PathBuf {
        self.anvil_dir().join("blocks.json")
    }

    pub fn save_blocks(&self) -> anyhow::Result<()> {
        let blocks_json = serde_json::to_string_pretty(&self.blocks)?;
        fs::write(self.blocks_file(), blocks_json)?;
        Ok(())
    }

    pub fn is_genesis(&self) -> bool {
        self.blocks.is_empty()
    }
}
