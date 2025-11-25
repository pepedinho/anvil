use std::{fs, path::Path};

use anyhow::Result;

use crate::config::Config;

impl Config {
    pub fn new(path: Option<&Path>) -> Result<Self> {
        let yaml = if let Some(p) = path {
            println!("debug: config path: {}", p.display());
            fs::read_to_string(p)?
        } else {
            fs::read_to_string(".anvil/anvil.yml")?
        };
        let config: Config = serde_yaml::from_str(&yaml)?;
        Ok(config)
    }
}
