#[test]
fn test_anvilcore_new_genesis() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config = crate::config::Config::default();
    let store = crate::store::mock::MockStore::new(temp_dir.path().to_string_lossy().to_string());

    let anvil = crate::core::AnvilCore::new(config, store, temp_dir.path().to_path_buf()).unwrap();

    assert!(anvil.is_genesis());
    assert_eq!(anvil.blocks.len(), 0);
}

#[test]
fn test_pack_reuse_block() {
    let temp_dir = tempfile::tempdir().unwrap();
    let store = crate::store::mock::MockStore::new(temp_dir.path().to_string_lossy().to_string());
    let config = crate::config::Config::default();

    let mut anvil =
        crate::core::AnvilCore::new(config, store, temp_dir.path().to_path_buf()).unwrap();

    let fake_bin = temp_dir.path().join("fake_bin");
    std::fs::write(&fake_bin, b"hello").unwrap();
    anvil.config.build.entrypoint = fake_bin;
    anvil.config.build.command = "echo Build".to_string();

    anvil.pack("0.0.1", false).unwrap();
    assert_eq!(anvil.blocks.len(), 1);

    anvil.pack("0.0.2", false).unwrap();
    assert_eq!(anvil.blocks.len(), 1);
}
