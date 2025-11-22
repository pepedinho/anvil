use std::path::PathBuf;

pub mod fs_store;
pub mod meta;
pub mod traits;

pub struct StoreRef {
    pub path: PathBuf,
    pub hash: String,
}
