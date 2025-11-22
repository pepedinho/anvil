use std::{fs, path::PathBuf, time::SystemTime};

use crate::{
    cli::{Cli, Commands},
    config::Config,
    store::{
        meta::{ArtefactType, Meta, get_last_commit},
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

    pub fn pack(&mut self) -> anyhow::Result<()> {
        if let Some(script) = &self.config.dependency_script {
            let status = std::process::Command::new("sh")
                .arg(script)
                .current_dir(&self.project_root)
                .status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Dependency script failed"));
            }
        }

        let entrypoint_path = self.project_root.join(&self.config.build.entrypoint);
        let artifact_bytes = std::fs::read(&entrypoint_path)?;

        let mut meta = Meta {
            artefact_hash: S::compute_hash(&artifact_bytes),
            artefact_type: ArtefactType::Bin,
            created_at: SystemTime::now(),
            git_commit: self.current_commit.clone().unwrap(),
            prev_block_hash: self.blocks.last().map(|b| b.block_hash.clone()),
            block_hash: String::new(),
            entrypoint: entrypoint_path.to_string_lossy().to_string(),
        };

        meta.block_hash = S::compute_block_hash(&meta);

        self.store.add_artifact(&artifact_bytes, &meta)?;
        self.blocks.push(meta);
        self.save_blocks()?;

        if self.is_genesis() {
            println!("Genesis block created!");
        } else {
            println!("New block packed!")
        }

        Ok(())
    }
}
