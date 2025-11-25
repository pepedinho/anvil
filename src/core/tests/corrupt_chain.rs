// #[test]
// fn test_missing_block_in_chain() {
//     let temp = tempfile::tempdir().unwrap();
//     let store = crate::store::mock::MockStore::new(temp.path().to_string_lossy().to_string());
//     let mut config = crate::config::Config::default();

//     let fake_bin = temp.path().join("bin");
//     config.build.entrypoint = fake_bin.clone();
//     config.build.command = "echo build".to_string();

//     let mut anvil = crate::core::AnvilCore::new(config, store, temp.path().to_path_buf()).unwrap();

//     // genesis (block 0)
//     std::fs::write(&fake_bin, b"hello").unwrap();
//     anvil.pack().unwrap();
//     assert_eq!(anvil.blocks.len(), 1);

//     // block 1
//     std::fs::write(&fake_bin, b"world").unwrap();
//     anvil.pack().unwrap();
//     assert_eq!(anvil.blocks.len(), 2);

//     anvil.blocks.remove(0);

//     assert!(anvil.validate_chain().is_err());
// }

#[test]
fn test_invalid_prev_block_hash() {
    let temp = tempfile::tempdir().unwrap();
    let store = crate::store::mock::MockStore::new(temp.path().to_string_lossy().to_string());
    let config = crate::config::Config::default();

    let fake_bin = temp.path().join("bin");

    let mut anvil = crate::core::AnvilCore::new(config, store, temp.path().to_path_buf()).unwrap();
    anvil.config.build.entrypoint = fake_bin.clone();
    anvil.config.build.command = "echo".to_string();

    std::fs::write(&fake_bin, b"hello").unwrap();
    anvil.pack("0.0.1").unwrap();
    std::fs::write(&fake_bin, b"world").unwrap();
    anvil.pack("0.0.2").unwrap();

    anvil.blocks[1].prev_block_hash = Some("FAKE_PREV_HASH".into());

    assert!(anvil.validate_chain().is_err());
}

// #[test]
// fn test_invalid_block_hash() {
//     let temp = tempfile::tempdir().unwrap();
//     let store = crate::store::mock::MockStore::new(temp.path().to_string_lossy().to_string());
//     let config = crate::config::Config::default();

//     let fake_bin = temp.path().join("bin");
//     std::fs::write(&fake_bin, b"hello").unwrap();

//     let mut anvil = crate::core::AnvilCore::new(config, store, temp.path().to_path_buf()).unwrap();
//     anvil.config.build.entrypoint = fake_bin.clone();
//     anvil.config.build.command = "echo".to_string();

//     anvil.pack().unwrap();

//     anvil.blocks[0].block_hash = "WRONG_HASH".into();

//     assert!(anvil.validate_chain().is_err());
// }

#[test]
fn test_block_out_of_order() {
    let temp = tempfile::tempdir().unwrap();
    let store = crate::store::mock::MockStore::new(temp.path().to_string_lossy().to_string());
    let config = crate::config::Config::default();

    let fake_bin = temp.path().join("bin");

    let mut anvil = crate::core::AnvilCore::new(config, store, temp.path().to_path_buf()).unwrap();
    anvil.config.build.entrypoint = fake_bin.clone();
    anvil.config.build.command = "echo".to_string();

    std::fs::write(&fake_bin, b"hello").unwrap();
    anvil.pack("0.0.1").unwrap();
    std::fs::write(&fake_bin, b"wonderful").unwrap();
    anvil.pack("0.0.2").unwrap();
    std::fs::write(&fake_bin, b"world").unwrap();
    anvil.pack("0.0.3").unwrap();

    anvil.blocks.reverse();

    assert!(anvil.validate_chain().is_err());
}
