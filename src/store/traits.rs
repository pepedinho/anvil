use anyhow::Result;

use crate::store::{StoreRef, meta::Meta};

pub trait Store {
    fn add_artifact(&self, artifact_byte: &[u8], meta: &Meta) -> Result<StoreRef>;
    fn get_artifact(&self, hash: &str) -> Option<StoreRef>;
}
