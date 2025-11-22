use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::store::{meta::Meta, traits::Store};

#[derive(Debug)]
pub enum StoreState {
    Genesis,
    Incremental,
}

#[derive(Debug)]
pub struct FsStore {
    root: PathBuf,
}

fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

impl FsStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let home = get_home_dir().unwrap();
        let path = home.join(path);
        println!("debug: path: {:#?}", path);

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        if !path.is_dir() {
            return Err(anyhow::anyhow!("Store path exists but is not a directory"));
        }
        Ok(Self { root: path })
    }

    pub fn get_path(path: &str) -> PathBuf {
        let home = get_home_dir().unwrap();
        home.join(path)
    }
}

impl Store for FsStore {
    fn add_artifact(&self, artifact_byte: &[u8], meta: &Meta) -> Result<()> {
        let path = self.root.join(&meta.block_hash);
        fs::write(path, artifact_byte)?;
        Ok(())
    }

    fn get_artifact(&self, hash: &str) -> Result<Vec<u8>> {
        let path = self.root.join(hash);
        let data = fs::read(path)?;
        Ok(data)
    }

    fn exists(&self, hash: &str) -> bool {
        self.root.join(hash).exists()
    }

    fn root(&self) -> &std::path::Path {
        &self.root
    }
}
