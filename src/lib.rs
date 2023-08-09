pub(crate) mod colors;
pub(crate) mod loggers;
pub(crate) mod prelude;
pub(crate) mod repository;

pub mod builder;
pub mod config;

use log::{Level, Log};
use loggers::Logger;

pub use crate::builder::*;

pub struct Purrfect {
    loggers: Vec<Logger>,
}

impl Log for Purrfect {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        for i in self.loggers.iter() {
            i.log(record)
        }
    }

    fn flush(&self) {
        for i in self.loggers.iter() {
            i.flush()
        }
    }
}
