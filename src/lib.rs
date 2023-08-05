pub mod builder;
pub mod colors;
pub mod config;
pub mod loggers;
pub mod prelude;
pub mod repository;

use std::io;

pub use crate::builder::*;

use log::{Level, LevelFilter, Log};
use loggers::console::Console;

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

pub fn setup() {
    let logger = Purrfect {
        loggers: vec![Console::new_boxed(Level::Info)],
    };

    let l = log::set_boxed_logger(Box::new(logger));

    if l.is_ok() {
        log::set_max_level(LevelFilter::Info)
    }
}
