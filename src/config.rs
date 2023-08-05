use std::fs::OpenOptions;

use log::Log;
use serde::{Deserialize, Serialize};

use crate::{
    colors::LevelColors,
    loggers::{Console, File},
    prelude::Wrapper,
    repository::logger_level::LoggerLevel,
};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub level_colors: LoggerColors,
    pub loggers: Vec<Logger>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LoggerColors {
    error: LevelColors,
    warn: LevelColors,
    info: LevelColors,
    debug: LevelColors,
    trace: LevelColors,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Logger {
    Console(ConsoleConfig),
    File(FileConfig),
}

impl Default for Logger {
    fn default() -> Self {
        Self::Console(ConsoleConfig {
            level: Wrapper(log::Level::Info),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConsoleConfig {
    level: LoggerLevel,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct FileConfig {
    path: String,
    level: LoggerLevel,
}

impl Logger {
    pub fn choose<'a>(&'a self) -> Box<dyn Log + '_> {
        match self {
            Logger::Console(c) => Console::new_boxed(c.level.0),
            Logger::File(c) => File::new_boxed(c.level.0, {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&c.path)
                    .unwrap()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Wrapper;

    use super::*;

    #[test]
    fn serialize_test() {
        #[warn(dead_code)]
        let cfg = Config {
            level_colors: LoggerColors {
                error: LevelColors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                warn: LevelColors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                info: LevelColors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                debug: LevelColors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                trace: LevelColors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
            },
            loggers: vec![Logger::File(FileConfig {
                path: "St.log".to_string(),
                level: Wrapper(log::Level::Trace),
            })],
        };

        let _ = std::fs::File::create("Purrfect.toml").unwrap();

        let content = toml::to_string_pretty(&cfg).unwrap();

        std::fs::write("Purrfect.toml", &content).unwrap();

        println!("{}", content)
    }

    #[test]
    #[ignore = "don't work with serialize_test in one time"]
    fn deserialize_test() {
        let file = std::fs::read_to_string("Purrfect.toml").unwrap();

        let content = toml::from_str::<Config>(&file).unwrap();

        println!("{:#?}", content)
    }
}
