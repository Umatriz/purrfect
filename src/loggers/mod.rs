pub mod console;
pub mod file;

pub(crate) use console::Console;
pub(crate) use file::File;
use log::Log;

pub enum Logger {
    Console(console::Console),
    File(file::File),
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        todo!()
    }

    fn log(&self, record: &log::Record) {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}
