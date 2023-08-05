use std::{
    io::{BufWriter, Write},
    sync::Mutex,
};

use log::{Level, Log};

#[derive(Debug)]
pub struct File {
    stream: Mutex<BufWriter<std::fs::File>>,
    level: Level,
}

impl Default for File {
    fn default() -> Self {
        Self {
            stream: Mutex::new(BufWriter::new(
                std::fs::File::create("default.log").unwrap(),
            )),
            level: Level::Info,
        }
    }
}

impl File {
    pub fn new(level: Level, file: std::fs::File) -> Self {
        Self {
            stream: Mutex::new(BufWriter::new(file)),
            level,
        }
    }

    pub fn new_boxed(level: Level, file: std::fs::File) -> Box<Self> {
        Box::new(Self::new(level, file))
    }
}

impl Log for File {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.stream.lock().unwrap_or_else(|e| e.into_inner());

            writeln!(writer, "{}", record.args()).unwrap();

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
