pub mod builder;
pub mod colors;
pub mod config;
pub mod loggers;
pub mod prelude;
pub mod repository;

use std::fs::OpenOptions;

use log::{Level, LevelFilter, Log};
use loggers::{console::Console, File};

pub use crate::builder::*;

pub struct Purrfect {
    loggers: Vec<Box<dyn Log>>,
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
