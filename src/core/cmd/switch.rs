use std::{
    env::home_dir,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};

use crate::{config::Config, core::AnvilCore, store::traits::Store};

#[derive(Serialize, Deserialize)]
pub struct MetaFile {
    repo_url: String,
    local_repo_path: PathBuf,
    current_version: String,
    current_commit: String,
}

impl<S: Store> AnvilCore<S> {
    pub fn switch(&mut self, name: &str, version: &str) -> anyhow::Result<()> {
        let repo_path = self.repo_install_path(name)?;
        let meta_file = self.get_meta_data(name)?;

        self.ensure_repo_cloned(&meta_file.repo_url, &repo_path)?;

        self.checkout_head(&repo_path)?;

        let blocks = Self::load_block_from_repo(&repo_path)?;
        let block = Self::resolve_version(&blocks, Some(version.to_string()))?;
        println!("debug: version find : {:#?}", block);
        let commit = block.git_commit.clone();

        self.checkout_commit(&repo_path, &commit)?;
        self.config = Config::new(Some(&repo_path.join(".anvil/anvil.yml")))
            .or_else(|_| Config::new(Some(&repo_path.join(".anvil/anvil.yaml"))))?;

        let bin_path = self.build_binary(&repo_path)?;
        let _final_bin = self.install_binary(name, &bin_path);

        self.update_meta(name, &meta_file.repo_url, &repo_path, &block)?;

        println!("{name} switched to {}", block.version);

        Ok(())
    }

    fn get_meta_data(&self, name: &str) -> anyhow::Result<MetaFile> {
        let meta_path = home_dir()
            .unwrap()
            .join(".anvil/meta")
            .join(format!("{name}.json"));

        let meta_cotent = fs::read_to_string(meta_path)?;
        let meta_file: MetaFile = serde_json::from_str(&meta_cotent)?;
        Ok(meta_file)
    }

    fn checkout_head(&self, repo_path: &Path) -> anyhow::Result<()> {
        // 1. On détermine la "branche courante" d'après Git.
        let output = Command::new("git")
            .args([
                "-C",
                repo_path.to_str().unwrap(),
                "rev-parse",
                "--abbrev-ref",
                "HEAD",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;

        let mut branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // 2. Si HEAD est détaché
        if branch == "HEAD" {
            // On essaye de revenir sur `main`, puis `master`
            for candidate in ["main", "master"] {
                let status = Command::new("git")
                    .args([
                        "-C",
                        repo_path.to_str().unwrap(),
                        "show-ref",
                        "--verify",
                        &format!("refs/heads/{candidate}"),
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()?;

                if status.success() {
                    branch = candidate.to_string();
                    break;
                }
            }
        }

        // 3. Checkout vers la branche trouvée
        Command::new("git")
            .args(["-C", repo_path.to_str().unwrap(), "checkout", &branch])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        Ok(())
    }
}
