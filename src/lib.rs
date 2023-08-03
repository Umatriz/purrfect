pub mod builder;
pub mod colors;
pub mod config;
pub mod loggers;
pub mod prelude;

pub use crate::builder::*;

use log::{Level, LevelFilter, Log};

struct Logger {
    loggers: Vec<Box<dyn Log>>,
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        for i in self.loggers.iter() {
            i.log(record)
        }
    }

    fn flush(&self) {}
}

struct Console;

impl Log for Console {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} [CONSOLE] - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

struct File;

impl Log for File {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} [FILE] - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

const CONSOLE: Console = Console;
const FILE: File = File;

pub fn setup() {
    let logger = Logger {
        loggers: vec![Box::new(CONSOLE), Box::new(FILE)],
    };

    let l = log::set_boxed_logger(Box::new(logger));

    if l.is_ok() {
        log::set_max_level(LevelFilter::Info)
    }
}
