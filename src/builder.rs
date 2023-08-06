use std::{marker::PhantomData, path::Path, rc::Rc};

use log::Log;

use crate::{config::Config, Purrfect};

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
    pub fn file<P: AsRef<Path>>(self, path: P) -> PurrfectBuilder<ConfigFile<P>> {
        PurrfectBuilder {
            config: ConfigFile::<P>(path),
        }
    }

    pub fn config(self, cfg: Config) -> PurrfectBuilder<Config> {
        PurrfectBuilder { config: cfg }
    }
}

impl<P: AsRef<Path>> PurrfectBuilder<ConfigFile<P>> {
    pub fn build(self) {
        // Panic if cannot read
        let file = std::fs::read_to_string(self.config.0).unwrap();

        // Panic if cannot deserialize
        let config = toml::from_str::<Config>(&file).unwrap();
        let config_iter = config.loggers.into_iter();

        let loggers = config_iter
            .map(|i| i.prepare())
            .collect::<Vec<Box<dyn Log>>>();

        let purr = Purrfect { loggers };

        let l = log::set_boxed_logger(Box::new(purr));

        if l.is_ok() {
            log::set_max_level(log::LevelFilter::Trace)
        }
    }
}

impl PurrfectBuilder<Config> {
    pub fn build(self) -> Purrfect {
        // TODO
        Purrfect { loggers: todo!() }
    }
}
