use std::env;

use anvil::{
    cli::{Cli, Commands},
    config::Config,
    core::AnvilCore,
    store::fs_store::FsStore,
};
use clap::Parser;

fn get_project_name() -> anyhow::Result<String> {
    let current = env::current_dir()?;
    if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
        Ok(name.to_string())
    } else {
        Err(anyhow::anyhow!("Cannot determine project folder name"))
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::new()?;
    let name = get_project_name()?;
    let store_path = FsStore::get_path(&format!(".anvil/store/{name}"));
    let store = FsStore::new(store_path)?;

    let mut anvil = AnvilCore::new(config, store, env::current_dir()?)?;
    dbg!(&anvil);
    anvil.interpret(&cli)?;

    Ok(())
}
