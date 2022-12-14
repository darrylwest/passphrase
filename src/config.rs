/// Configuration for generating PassPhrases.
///
///
use std::fs;

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub seed: Option<usize>,
    pub word_count: u8,
    pub limit: usize,
}

impl Config {
    pub fn read_config(filename: &str) -> Result<Config> {
        let text = fs::read_to_string(filename)?;

        let config = toml::from_str(&text).unwrap();
        info!("{:?}", config);

        Ok(config)
    }

    /// create a new config struct with default values
    pub fn new() -> Config {
        let name = Self::default_name();
        Config::with_values(&name, None, 12_u8, 20_usize)
    }

    /// create config with the seed
    pub fn with_seed(seed: Option<usize>) -> Config {
        let name = Self::default_name();
        Config::with_values(&name, seed, 12_u8, 20_usize)
    }

    /// construct with seed and word count
    pub fn with_seed_and_word_count(seed: Option<usize>, word_count: u8) -> Config {
        let name = Self::default_name();
        Config::with_values(&name, seed, word_count, 20_usize)
    }

    /// construct with all values
    pub fn with_values(name: &str, seed: Option<usize>, word_count: u8, limit: usize) -> Config {
        Config {
            name: name.to_owned(),
            seed,
            word_count,
            limit,
        }
    }

    /// return the default config name, e.g. system time as seconds
    pub fn default_name() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("should get the current system time");

        format!("{}", now.as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_name() {
        let name = Config::default_name();
        println!("{}", name);
        assert!(name.len() > 8)
    }

    #[test]
    fn read_config() {
        let filename = "tests/config.toml";
        let config = Config::read_config(filename).unwrap();

        assert_eq!(config.name, "plaza.local");
        assert_eq!(config.seed, Some(1234));
        assert_eq!(config.word_count, 10);
        assert_eq!(config.limit, 100);
    }

    #[test]
    fn read_config_no_seed() {
        let filename = "tests/config-no-seed.toml";
        let config = Config::read_config(filename).unwrap();

        assert_eq!(config.name, "tiburon.local");
        assert_eq!(config.seed, None);
        assert_eq!(config.word_count, 20);
        assert_eq!(config.limit, 10);
    }
}
