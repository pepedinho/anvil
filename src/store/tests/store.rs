#[test]
fn test_fsstore_add_and_get() {
    use crate::store::{
        fs_store::FsStore,
        meta::{ArtefactType, Meta},
        traits::Store,
    };
    use std::time::SystemTime;
    let temp_dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(temp_dir.path()).unwrap();

    let data = b"hello world";
    let hash = FsStore::compute_hash(data);

    let meta = Meta {
        artefact_hash: hash.clone(),
        artefact_type: ArtefactType::Bin,
        created_at: SystemTime::now(),
        git_commit: "abc1234".to_string(),
        prev_block_hash: None,
        block_hash: hash.clone(),
        entrypoint: "test".to_string(),
        version: "0.0.1".to_string(),
    };

    store.add_artifact(data, &meta).unwrap();
    let read = store.get_artifact(&hash).unwrap();

    assert_eq!(data.to_vec(), read);
}
