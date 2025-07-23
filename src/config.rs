use std::fs;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::FrankmarkResult;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    pub directories: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub github_url: Option<String>,
}

pub fn parse_config(config_path: &str) -> FrankmarkResult<Config> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
} 