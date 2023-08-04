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

impl<A> PurrfectBuilder<A> {
    pub fn config<P: AsRef<Path>>(self, path: P) -> PurrfectBuilder<ConfigFile<P>> {
        PurrfectBuilder {
            config: ConfigFile::<P>(path),
        }
    }
}

impl<P: AsRef<Path>> PurrfectBuilder<ConfigFile<P>> {
    pub fn build(self) -> Purrfect {
        Purrfect { loggers: todo!() }
    }
}
