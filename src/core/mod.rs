use std::{env, fs, path::PathBuf};

pub mod cmd;
pub mod tests;

use crate::{
    cli::{Cli, Commands},
    config::Config,
    store::{
        fs_store::FsStore,
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

fn get_project_name() -> anyhow::Result<String> {
    let current = env::current_dir()?;
    if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
        Ok(name.to_string())
    } else {
        Err(anyhow::anyhow!("Cannot determine project folder name"))
    }
}

impl<S: Store> AnvilCore<S> {
    pub fn new(config: Option<Config>, store: S, project_root: PathBuf) -> anyhow::Result<Self> {
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
            config: config.unwrap_or_default(),
            store,
            blocks,
            current_commit: last_commit,
            project_root,
        })
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

    pub fn validate_chain(&self) -> anyhow::Result<()> {
        for i in 1..self.blocks.len() {
            let prev = &self.blocks[i - 1];
            let curr = &self.blocks[i];

            if curr.prev_block_hash.as_deref() != Some(&prev.block_hash) {
                return Err(anyhow::anyhow!(
                    "Invalid chain: block {} does not correctly reference previous block {}",
                    i,
                    i - 1,
                ));
            }

            let expected_hash = S::compute_block_hash(curr);
            if expected_hash != curr.block_hash {
                return Err(anyhow::anyhow!(
                    "Invalid block hash for block {}: expected {}, found {}",
                    i,
                    expected_hash,
                    curr.block_hash
                ));
            }
        }
        Ok(())
    }
}

pub fn interpret(cli: &Cli) -> anyhow::Result<()> {
    let name = get_project_name()?;
    let store_path = FsStore::get_path(&format!(".anvil/store/{name}"));
    let store = FsStore::new(store_path)?;
    match &cli.command {
        Commands::Pack { v, tag } => {
            let config = Config::new(None)?;
            AnvilCore::new(Some(config), store, env::current_dir()?)?.pack(v, *tag)
        }
        Commands::Install { url, version } => {
            AnvilCore::new(None, store, env::current_dir()?)?.install(url, version.clone())
        }
        Commands::Switch {
            project: _,
            version: _,
        } => Ok(()),
    }
}
