use serde::{Deserialize, Serialize};

use crate::colors::Color;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    error: Color,
    warn: Color,
    info: Color,
    debug: Color,
    trace: Color,
}

#[test]
fn serialize_test() {
    let cfg = Config {
        error: Color {
            background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
            color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
        },
        warn: Color {
            background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
            color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
        },
        info: Color {
            background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
            color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
        },
        debug: Color {
            background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
            color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
        },
        trace: Color {
            background: crate::prelude::Wrapper(owo_colors::AnsiColors::Blue),
            color: crate::prelude::Wrapper(owo_colors::AnsiColors::Yellow),
        },
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
