use std::{
    io::{BufWriter, Write},
    sync::Mutex,
};

use log::{Level, Log};

use crate::repository::pattern::Message;

#[derive(Debug)]
pub struct File {
    stream: Mutex<BufWriter<std::fs::File>>,
    level: Level,
    pattern: Message,
}

impl Default for File {
    fn default() -> Self {
        Self {
            stream: Mutex::new(BufWriter::new(
                std::fs::File::create("default.log").unwrap(),
            )),
            level: Level::Info,
            pattern: Message::default(),
        }
    }
}

impl File {
    pub fn new(level: Level, file: std::fs::File, pattern: Message) -> Self {
        Self {
            stream: Mutex::new(BufWriter::new(file)),
            level,
            pattern,
        }
    }

    pub fn new_boxed(level: Level, file: std::fs::File, pattern: Message) -> Box<Self> {
        Box::new(Self::new(level, file, pattern))
    }
}

impl Log for File {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.stream.lock().unwrap_or_else(|e| e.into_inner());

            writeln!(writer, "{}", self.pattern.format_parsed(record)).unwrap();

            writer.flush().unwrap();
        }
    }

    fn flush(&self) {
        let _ = self
            .stream
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .flush();
    }
}
