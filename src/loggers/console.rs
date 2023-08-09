use std::io::{self, Write};

use log::{Level, Log};

use crate::repository::pattern::MessagePattern;

#[derive(Debug)]
pub struct Console {
    stream: io::Stdout,
    level: Level,
    pattern: MessagePattern,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            stream: io::stdout(),
            level: Level::Info,
            pattern: MessagePattern::default(),
        }
    }
}

impl Console {
    pub fn new(level: Level, pattern: MessagePattern) -> Self {
        Self {
            stream: io::stdout(),
            level,
            pattern,
        }
    }

    pub fn new_boxed(level: Level, pattern: MessagePattern) -> Box<Self> {
        Box::new(Self::new(level, pattern))
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
