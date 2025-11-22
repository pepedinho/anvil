use anyhow::Result;
use sha2::{Digest, Sha256};

use crate::store::meta::Meta;

pub trait Store {
    fn add_artifact(&self, artifact_byte: &[u8], meta: &Meta) -> Result<()>;
    fn get_artifact(&self, hash: &str) -> Result<Vec<u8>>;
    fn exists(&self, hash: &str) -> bool;
    fn root(&self) -> &std::path::Path;

    fn compute_hash(data: &[u8]) -> String
    where
        Self: Sized,
    {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn compute_block_hash(meta: &Meta) -> String
    where
        Self: Sized,
    {
        let json = serde_json::to_string(meta).unwrap();
        Self::compute_hash(json.as_bytes())
    }
}
