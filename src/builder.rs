use std::{marker::PhantomData, path::Path};

use crate::{config::Config, Purrfect};

#[derive(Default, Clone)]
struct NoConfig;

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
    pub fn build(self) -> Purrfect {
        // TODO
        Purrfect { loggers: todo!() }
    }
}

impl PurrfectBuilder<Config> {
    pub fn build(self) -> Purrfect {
        // TODO
        Purrfect { loggers: todo!() }
    }
}
