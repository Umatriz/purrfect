use std::io::{self, Write};

use log::{Level, Log};

use crate::repository::pattern::Message;

#[derive(Debug)]
pub struct Console {
    stream: io::Stdout,
    level: Level,
    pattern: Message,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            stream: io::stdout(),
            level: Level::Info,
            pattern: Message::default(),
        }
    }
}

impl Console {
    pub fn new(level: Level, pattern: Message) -> Self {
        Self {
            stream: io::stdout(),
            level,
            pattern,
        }
    }

    pub fn new_boxed(level: Level, pattern: Message) -> Box<Self> {
        Box::new(Self::new(level, pattern))
    }
}

impl Log for Console {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            writeln!(self.stream.lock(), "{}", self.pattern.format_parsed(record)).unwrap();
        }
    }

    fn flush(&self) {
        let _ = self.stream.lock().flush();
    }
}
