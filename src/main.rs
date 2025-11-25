use anvil::{cli::Cli, core::interpret};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // let config = Config::new()?;
    // let name = get_project_name()?;
    // let store_path = FsStore::get_path(&format!(".anvil/store/{name}"));
    // let store = FsStore::new(store_path)?;

    // let mut anvil = AnvilCore::new(config, store, env::current_dir()?)?;
    // dbg!(&anvil);
    interpret(&cli)?;

    Ok(())
}
