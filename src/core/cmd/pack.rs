use std::time::SystemTime;

use crate::{
    core::{AnvilCore, cmd::run_step},
    store::{
        meta::{ArtefactType, Meta},
        traits::Store,
    },
};

impl<S: Store> AnvilCore<S> {
    pub fn pack(&mut self, v: &str, tag: bool) -> anyhow::Result<()> {
        if let Some(script) = &self.config.dependency_script {
            let status = std::process::Command::new("sh")
                .arg(script)
                .current_dir(&self.project_root)
                .status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Dependency script failed"));
            }
        }

        run_step(
            &self.config.build.command,
            Some(&self.project_root),
            "green",
            "Forging...",
            "Artefact forged !",
        )?;

        // run_build_cmd(&self.config.build, &self.project_root)?;

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

        if tag {
            self.create_git_tag(v)
                .map_err(|e| anyhow::anyhow!("block packed but failed to create git tag: {e}"))?;
        }

        if self.is_genesis() {
            println!("Genesis block created!");
        } else {
            println!("New block packed!")
        }

        Ok(())
    }

    fn create_git_tag(&self, version: &str) -> anyhow::Result<()> {
        let status = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .current_dir(&self.project_root)
            .output()?;

        if !status.status.success() {
            anyhow::bail!("Cannot create tag: current directory is not a git repository");
        }

        let tag_check = std::process::Command::new("git")
            .arg("tag")
            .arg("--list")
            .arg(version)
            .current_dir(&self.project_root)
            .output()?;

        if !tag_check.stdout.is_empty() {
            anyhow::bail!("Git tag '{}' already exist", version);
        }

        let output = std::process::Command::new("git")
            .arg("tag")
            .arg("-a")
            .arg(version)
            .arg("-m")
            .arg(format!("Anvil release {version}"))
            .current_dir(&self.project_root)
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to create git tag: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!("Git tag '{version}' created successfully!");

        Ok(())
    }
}
