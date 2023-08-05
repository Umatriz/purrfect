use std::io::{self, Write};

use log::{Level, Log};

#[derive(Debug)]
pub struct Console {
    stream: io::Stdout,
    // colors: LoggerColors,
    level: Level,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            stream: io::stdout(),
            level: Level::Info,
        }
    }
}

impl Console {
    pub fn new(level: Level) -> Self {
        Self {
            stream: io::stdout(),
            level,
        }
    }

    pub fn new_boxed(level: Level) -> Box<Self> {
        Box::new(Self {
            stream: io::stdout(),
            level,
        })
    }
}

impl Log for Console {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            write!(self.stream.lock(), "{}", record.args()).unwrap();
        }
    }

    fn flush(&self) {
        let _ = self.stream.lock().flush();
    }
}
