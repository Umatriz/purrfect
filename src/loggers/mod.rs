pub mod console;
pub mod file;

pub(crate) use console::Console;
pub(crate) use file::File;
use log::Log;

pub enum Logger {
    Console(console::Console),
    File(file::File),
}

impl Default for Logger {
    fn default() -> Self {
        Logger::Console(Console::default())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        match self {
            Logger::Console(logger) => logger.enabled(metadata),
            Logger::File(logger) => logger.enabled(metadata),
        }
    }

    fn log(&self, record: &log::Record) {
        match self {
            Logger::Console(logger) => logger.log(record),
            Logger::File(logger) => logger.log(record),
        }
    }

    fn flush(&self) {
        match self {
            Logger::Console(logger) => logger.flush(),
            Logger::File(logger) => logger.flush(),
        }
    }
}
