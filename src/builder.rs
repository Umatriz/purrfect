use std::marker::PhantomData;

use crate::{
    config::{Config, LoggerColors},
    loggers::Logger,
    prelude::Result,
    Purrfect,
};

pub struct NoLogger;
pub struct WithLogger;

#[derive(Default)]
pub struct PurrfectBuilder<State = NoLogger> {
    loggers: Vec<Logger>,
    level_colors: LoggerColors,
    _state: PhantomData<State>,
}

impl PurrfectBuilder {
    pub fn new() -> PurrfectBuilder {
        PurrfectBuilder {
            loggers: vec![],
            level_colors: Default::default(),
            _state: PhantomData,
        }
    }

    pub fn colors(&self, colors: LoggerColors) -> PurrfectBuilder {
        PurrfectBuilder {
            loggers: self.loggers,
            level_colors: colors,
            _state: PhantomData,
        }
    }

    pub fn add_logger(&self, logger: Logger) -> PurrfectBuilder<WithLogger> {
        let mut loggers = self.loggers;
        loggers.push(logger);

        PurrfectBuilder {
            loggers,
            level_colors: self.level_colors,
            _state: PhantomData,
        }
    }
}

impl<State> PurrfectBuilder<State> {
    fn pre_build(self) -> Result<()> {
        let purr = Purrfect {
            loggers: self.loggers,
        };

        let l = log::set_boxed_logger(Box::new(purr));

        if l.is_ok() {
            log::set_max_level(log::LevelFilter::Trace);
        }

        Ok(())
    }
}

impl PurrfectBuilder<WithLogger> {
    pub fn build(self) -> Result<()> {
        Self::pre_build(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        PurrfectBuilder::new().add_logger(Logger::default()).build();
    }
}
