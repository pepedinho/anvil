use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Install {
        url: String,
        #[arg(short = 'v', long)]
        version: Option<String>,
    },
    Pack {
        v: String,
        #[arg(short = 't', long)]
        tag: bool,
    },
    Switch {
        project: String,
        version: String,
    },
}
