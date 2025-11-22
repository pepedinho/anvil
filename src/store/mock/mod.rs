use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::store::traits::Store;

pub struct MockStore {
    root_path: String,
    artifacts: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MockStore {
    pub fn new(root_path: impl Into<String>) -> Self {
        Self {
            root_path: root_path.into(),
            artifacts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Store for MockStore {
    fn add_artifact(
        &self,
        artifact_byte: &[u8],
        meta: &crate::store::meta::Meta,
    ) -> anyhow::Result<()> {
        let mut artefacts = self.artifacts.lock().unwrap();
        artefacts.insert(meta.block_hash.clone(), artifact_byte.to_vec());
        Ok(())
    }

    fn get_artifact(&self, hash: &str) -> anyhow::Result<Vec<u8>> {
        let artifacts = self.artifacts.lock().unwrap();
        Ok(artifacts.get(hash).cloned().unwrap_or_default())
    }

    fn exists(&self, hash: &str) -> bool {
        let artifacts = self.artifacts.lock().unwrap();
        artifacts.contains_key(hash)
    }

    fn root(&self) -> &std::path::Path {
        Path::new(&self.root_path)
    }

    fn compute_hash(data: &[u8]) -> String
    where
        Self: Sized,
    {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn compute_block_hash(meta: &super::meta::Meta) -> String
    where
        Self: Sized,
    {
        let json = serde_json::to_string(meta).unwrap();
        Self::compute_hash(json.as_bytes())
    }
}
