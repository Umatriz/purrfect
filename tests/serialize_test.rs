use purrfect::{colors::Color, config::*};
use std::io::Write;

#[test]
fn serialize_test() {
    let cfg = Config {
        level: LevelColors {
            error: Color {
                background: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                color: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
            },
            warn: Color {
                background: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                color: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
            },
            info: Color {
                background: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                color: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
            },
            debug: Color {
                background: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                color: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
            },
            trace: Color {
                background: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Blue),
                color: purrfect::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
            },
        },
    };

    let file = std::fs::File::create("cfg.toml").unwrap();

    let content = toml::to_string_pretty(&cfg).unwrap();

    std::fs::write("cfg.toml", &content).unwrap();

    println!("{content}")
}
