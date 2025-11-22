use crate::store::{fs_store::FsStore, meta::Meta};

pub struct AnvilCore {
    pub config: Config,
    pub store: FsStore,
    pub blocks: Vec<Meta>,
    pub current_commit: Option<String>,
}
