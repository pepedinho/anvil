use std::env;

use anvil::{
    cli::{Cli, Commands},
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

    match cli.command {
        Commands::Pack => {
            let name = get_project_name()?;
            FsStore::new(format!(".anvil/store/{name}"))?;
        }
        _ => {}
    }
    println!("receive: {:#?}", cli.command);
    Ok(())
}
