use std::time::SystemTime;

use crate::{
    core::{AnvilCore, cmd::run_build_cmd},
    store::{
        meta::{ArtefactType, Meta},
        traits::Store,
    },
};

impl<S: Store> AnvilCore<S> {
    pub fn pack(&mut self, v: &str) -> anyhow::Result<()> {
        if let Some(script) = &self.config.dependency_script {
            let status = std::process::Command::new("sh")
                .arg(script)
                .current_dir(&self.project_root)
                .status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Dependency script failed"));
            }
        }

        run_build_cmd(&self.config.build, &self.project_root)?;

        let entrypoint_path = self.project_root.join(&self.config.build.entrypoint);
        let artifact_bytes = std::fs::read(&entrypoint_path)?;
        let artefact_hash = S::compute_hash(&artifact_bytes);

        if let Some(existing_block) = self
            .blocks
            .iter()
            .find(|b| b.artefact_hash == artefact_hash)
        {
            println!(
                "Artefact unchanged, reusing existing block: {}",
                existing_block.block_hash
            );
            return Ok(());
        }

        let mut meta = Meta {
            artefact_hash,
            artefact_type: ArtefactType::Bin,
            created_at: SystemTime::now(),
            git_commit: self.current_commit.clone().unwrap(),
            prev_block_hash: self.blocks.last().map(|b| b.block_hash.clone()),
            block_hash: String::new(),
            entrypoint: self.config.build.entrypoint.to_string_lossy().to_string(),
            version: v.to_string(),
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
