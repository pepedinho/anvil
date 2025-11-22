use std::fs;

use anyhow::Result;

use crate::config::Config;

impl Config {
    pub fn new() -> Result<Self> {
        let yaml = fs::read_to_string(".anvil/anvil.yaml")?;
        let config: Config = serde_yaml::from_str(&yaml)?;
        Ok(config)
    }
}
