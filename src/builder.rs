use std::path::Path;

use crate::{config::Config, loggers::Logger, prelude::Result, Purrfect};

pub mod units {
    //! This module provides unit structs for `PurrfectBuilder`

    #[derive(Default, Clone)]
    pub struct NoConfig;

    #[derive(Default, Clone)]
    pub struct ConfigFile<P: AsRef<std::path::Path>>(pub P);
}

#[derive(Default, Clone)]
pub struct PurrfectBuilder<A> {
    config: A,
}

impl PurrfectBuilder<units::NoConfig> {
    pub fn new() -> Self {
        PurrfectBuilder::default()
    }
}

impl PurrfectBuilder<units::NoConfig> {
    pub fn config<P: AsRef<Path>>(self, path: P) -> PurrfectBuilder<units::ConfigFile<P>> {
        PurrfectBuilder {
            config: units::ConfigFile::<P>(path),
        }
    }

    pub fn custom_config(self, cfg: Config) -> PurrfectBuilder<Config> {
        PurrfectBuilder { config: cfg }
    }
}

impl<A> PurrfectBuilder<A> {
    fn pre_build(config: Config) -> Result<()> {
        let config_iter = config.loggers.iter();

        let loggers = config_iter
            .map(|i| i.prepare(&config))
            .filter_map(|i| i.ok())
            .collect::<Vec<Logger>>();

        let purr = Purrfect { loggers };

        let l = log::set_boxed_logger(Box::new(purr));

        if l.is_ok() {
            log::set_max_level(log::LevelFilter::Trace);
        }

        Ok(())
    }
}

impl<P: AsRef<Path>> PurrfectBuilder<units::ConfigFile<P>> {
    pub fn build(self) -> Result<()> {
        let file = std::fs::read_to_string(self.config.0)?;

        let config = toml::from_str::<Config>(&file)?;

        Self::pre_build(config)
    }
}

impl PurrfectBuilder<Config> {
    pub fn build(self) -> Result<()> {
        Self::pre_build(self.config)
    }
}
