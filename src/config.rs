use serde::{Deserialize, Serialize};

use crate::{colors::LevelColors, repository::logger_level::LoggerLevel};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Config {
    level_colors: LoggerColors,
    loggers: Vec<Logger>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct LoggerColors {
    error: LevelColors,
    warn: LevelColors,
    info: LevelColors,
    debug: LevelColors,
    trace: LevelColors,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Logger {
    #[serde(rename = "type")]
    _type: LoggerType,
    level: LoggerLevel,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub enum LoggerType {
    #[default]
    Console,
    File,
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
            loggers: vec![Logger {
                _type: LoggerType::Console,
                level: Wrapper(log::LevelFilter::Debug),
            }],
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
