use std::{
    env::home_dir,
    path::{Path, PathBuf},
};

use crate::{
    core::AnvilCore,
    store::{meta::Meta, traits::Store},
};

impl<S: Store> AnvilCore<S> {
    pub fn install(&self, url: &str, version: Option<String>) -> anyhow::Result<()> {
        let project_name = Self::extract_project_name(url)?;
        let repo_path = self.repo_install_path(&project_name)?;

        self.ensure_repo_cloned(url, &repo_path)?;

        let blocks = Self::load_block_from_repo(&repo_path)?;
        let block = Self::resolve_version(&blocks, version)?;
        let commit = block.git_commit.clone();

        self.checkout_commit(&repo_path, &commit)?;

        let bin_path = self.build_binary(&repo_path)?;
        let final_bin = self.install_binary(&project_name, &bin_path)?;

        self.update_meta(&project_name, url, &repo_path, block)?;

        println!(
            "Installed {} ({}) at {}",
            project_name,
            block.version,
            final_bin.display()
        );
        Ok(())
    }

    fn extract_project_name(repo_url: &str) -> anyhow::Result<String> {
        let name = repo_url
            .rsplit('/')
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid repo URL"))?
            .replace(".git", "");

        Ok(name)
    }

    fn repo_install_path(&self, name: &str) -> anyhow::Result<PathBuf> {
        let path = home_dir().unwrap().join(".anvil/repo"); //TODO: remove unwrap()

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(path.join(name))
    }

    fn ensure_repo_cloned(&self, url: &str, path: &Path) -> anyhow::Result<()> {
        if path.exists() {
            // update
            std::process::Command::new("git")
                .args(["-C", path.to_str().unwrap(), "fetch"])
                .status()?;
        } else {
            // clone
            std::process::Command::new("git")
                .args(["clone", url, path.to_str().unwrap()])
                .status()?;
        }

        Ok(())
    }

    fn load_block_from_repo(repo_path: &Path) -> anyhow::Result<Vec<Meta>> {
        let block_path = repo_path.join(".anvil/blocks.json");
        let content = std::fs::read_to_string(block_path)?;
        let blocks: Vec<Meta> = serde_json::from_str(&content)?;
        Ok(blocks)
    }

    fn resolve_version(blocks: &[Meta], version: Option<String>) -> anyhow::Result<&Meta> {
        if let Some(v) = version {
            blocks
                .iter()
                .find(|b| b.version == v)
                .ok_or_else(|| anyhow::anyhow!("Version {v} not found"))
        } else {
            blocks
                .last()
                .ok_or_else(|| anyhow::anyhow!("No blocks available"))
        }
    }

    pub fn checkout_commit(&self, repo_path: &Path, commit: &str) -> anyhow::Result<()> {
        std::process::Command::new("git")
            .args(["-C", repo_path.to_str().unwrap(), "checkout", commit])
            .status()?;
        Ok(())
    }

    pub fn build_binary(&self, repo_path: &PathBuf) -> anyhow::Result<PathBuf> {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.config.build.command)
            .current_dir(repo_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Build failed");
        }

        Ok(repo_path.join(&self.config.build.entrypoint))
    }

    fn install_binary(&self, name: &str, compiled_bin: &PathBuf) -> anyhow::Result<PathBuf> {
        let install_path = home_dir().unwrap().join(".anvil/bin").join(name);

        std::fs::create_dir_all(install_path.parent().unwrap())?;
        std::fs::copy(compiled_bin, &install_path)?;

        Ok(install_path)
    }

    fn update_meta(
        &self,
        name: &str,
        url: &str,
        repo_path: &PathBuf,
        block: &Meta,
    ) -> anyhow::Result<()> {
        let meta_path = home_dir()
            .unwrap()
            .join(".anvil/meta")
            .join(format!("{name}.json"));

        std::fs::create_dir_all(meta_path.parent().unwrap())?;

        let meta = serde_json::json!({
            "repo_url": url,
            "local_repo_path": repo_path,
            "current_version": block.version,
            "current_commit": block.git_commit,
        });

        std::fs::write(meta_path, serde_json::to_string_pretty(&meta)?)?;
        Ok(())
    }
}
