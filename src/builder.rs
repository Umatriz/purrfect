use std::path::Path;

use log::Log;

use crate::{config::Config, prelude::Result, Purrfect};

#[derive(Default, Clone)]
pub struct NoConfig;

#[derive(Default, Clone)]
pub struct ConfigFile<P: AsRef<Path>>(P);

#[derive(Default, Clone)]
pub struct PurrfectBuilder<A> {
    config: A,
}

impl PurrfectBuilder<NoConfig> {
    pub fn new() -> Self {
        PurrfectBuilder::default()
    }
}

impl PurrfectBuilder<NoConfig> {
    pub fn config<P: AsRef<Path>>(self, path: P) -> PurrfectBuilder<ConfigFile<P>> {
        PurrfectBuilder {
            config: ConfigFile::<P>(path),
        }
    }

    pub fn custom_config(self, cfg: Config) -> PurrfectBuilder<Config> {
        PurrfectBuilder { config: cfg }
    }
}

impl<A> PurrfectBuilder<A> {
    fn pre_build(config: Config) -> Result<()> {
        let config_iter = config.loggers.into_iter();

        let loggers = config_iter
            .map(|i| i.prepare())
            .filter_map(|i| i.ok())
            .collect::<Vec<Box<dyn Log>>>();

        let purr = Purrfect { loggers };

        let l = log::set_boxed_logger(Box::new(purr));

        if l.is_ok() {
            log::set_max_level(log::LevelFilter::Trace);
        }

        Ok(())
    }
}

impl<P: AsRef<Path>> PurrfectBuilder<ConfigFile<P>> {
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
