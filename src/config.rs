use std::{fs::OpenOptions, path::PathBuf};

use log::Log;
use serde::{Deserialize, Serialize};

use crate::{
    colors::Colors,
    loggers::{Console, File, Logger},
    prelude::*,
    repository::{logger_level::LoggerLevel, pattern::MessagePattern},
};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub level_colors: LoggerColors,
    pub loggers: Vec<LoggerConfig>,
    pub root: Root,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Root {
    pattern: String,
    time: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LoggerColors {
    pub error: Colors,
    pub warn: Colors,
    pub info: Colors,
    pub debug: Colors,
    pub trace: Colors,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LoggerConfig {
    Console(ConsoleConfig),
    File(FileConfig),
}

impl Default for LoggerConfig {
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

impl LoggerConfig {
    pub fn prepare(self, cfg: &Config) -> Result<Logger> {
        match self {
            LoggerConfig::Console(c) => Ok(Logger::Console(Console::new(
                c.level.0,
                MessagePattern::new(
                    cfg.root.pattern,
                    cfg.root.time,
                    true,
                    Some(cfg.level_colors),
                ),
            ))),
            LoggerConfig::File(c) => {
                if let Some(path) = PathBuf::from(&c.path).parent() {
                    std::fs::create_dir_all(path)?
                }

                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(c.path)?;

                Ok(Logger::File(File::new(c.level.0, file)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Wrapper;

    use super::*;

    #[test]
    #[ignore = "don't work with deserialize_test in one time"]
    fn serialize_test() {
        #[warn(dead_code)]
        let cfg = Config {
            level_colors: LoggerColors {
                error: Colors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                warn: Colors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                info: Colors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                debug: Colors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
                trace: Colors {
                    background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                    color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
                },
            },
            loggers: vec![
                Logger::File(FileConfig {
                    path: "St.log".to_string(),
                    level: Wrapper(log::Level::Trace),
                }),
                Logger::Console(ConsoleConfig {
                    level: Wrapper(log::Level::Trace),
                }),
            ],
            root: todo!(),
        };

        let _ = std::fs::File::create("Purrfect.toml").unwrap();

        let content = toml::to_string_pretty(&cfg).unwrap();

        std::fs::write("Purrfect.toml", &content).unwrap();

        println!("{}", content)
    }

    #[test]
    fn deserialize_test() {
        let file = std::fs::read_to_string("Purrfect.toml").unwrap();

        let content = toml::from_str::<Config>(&file).unwrap();

        println!("{:#?}", content)
    }
}
