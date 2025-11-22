use std::path::PathBuf;

use anyhow::Result;

pub mod fs_store;
pub mod meta;
pub mod traits;

pub struct StoreRef {
    pub path: PathBuf,
    pub hash: String,
}
