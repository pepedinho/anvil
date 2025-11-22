use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub enum StoreState {
    Genesis,
    Incremental,
}

pub struct FsStore {
    root: PathBuf,
    state: StoreState,
}

fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

impl FsStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let home = get_home_dir().unwrap();
        let path = home.join(path);
        let mut state = StoreState::Incremental;
        println!("debug: path: {:#?}", path);

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
            state = StoreState::Genesis;
        }

        if !path.is_dir() {
            return Err(anyhow::anyhow!("Store path exists but is not a directory"));
        }
        Ok(Self { root: path, state })
    }
}
