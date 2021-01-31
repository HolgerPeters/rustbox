use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use slog::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub verbosity: Option<usize>,
    pub source: String,
    pub destination: String,
}

impl Config {
    /// Reads a config file (TOML) format from `path` and overwrites the verbosity setting if not configured in the config.
    pub fn from_path(verbosity: usize, path: std::path::PathBuf) -> Self {
        let mut config_toml = String::new();
        let mut file = File::open(&path).unwrap();
        file.read_to_string(&mut config_toml).unwrap();
        let mut config: Config = toml::from_str(&config_toml).unwrap();

        config.verbosity = config.verbosity.or_else(|| Some(verbosity));
        config
    }

    pub fn logger(&self) -> slog::Logger {
        let build_id = build_id::get().to_hyphenated().to_string();
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain, o!("build-id" => build_id))
    }
}
