use owo_colors::AnsiColors;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::prelude::Wrapper;

pub type LoggerColor = Wrapper<owo_colors::AnsiColors>;

impl Default for LoggerColor {
    fn default() -> Self {
        Self(owo_colors::AnsiColors::Default)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = LoggerColor;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.as_str() {
            "Black" => Ok(Wrapper(AnsiColors::Black)),
            "Blue" => Ok(Wrapper(AnsiColors::Blue)),
            "BrightBlack" => Ok(Wrapper(AnsiColors::BrightBlack)),
            "BrightBlue" => Ok(Wrapper(AnsiColors::BrightBlue)),
            "BrightCyan" => Ok(Wrapper(AnsiColors::BrightCyan)),
            "BrightGreen" => Ok(Wrapper(AnsiColors::BrightGreen)),
            "BrightMagenta" => Ok(Wrapper(AnsiColors::BrightMagenta)),
            "BrightRed" => Ok(Wrapper(AnsiColors::BrightRed)),
            "BrightWhite" => Ok(Wrapper(AnsiColors::BrightWhite)),
            "BrightYellow" => Ok(Wrapper(AnsiColors::BrightYellow)),
            "Cyan" => Ok(Wrapper(AnsiColors::Cyan)),
            "Default" => Ok(Wrapper(AnsiColors::Default)),
            "Green" => Ok(Wrapper(AnsiColors::Green)),
            "Magenta" => Ok(Wrapper(AnsiColors::Magenta)),
            "Red" => Ok(Wrapper(AnsiColors::Red)),
            "White" => Ok(Wrapper(AnsiColors::White)),
            "Yellow" => Ok(Wrapper(AnsiColors::Yellow)),
            _ => Err(serde::de::Error::unknown_variant(
                &v,
                &[
                    "Black",
                    "Blue",
                    "BrightBlack",
                    "BrightBlue",
                    "BrightCyan",
                    "BrightGreen",
                    "BrightMagenta",
                    "BrightRed",
                    "BrightWhite",
                    "BrightYellow",
                    "Cyan",
                    "Default",
                    "Green",
                    "Magenta",
                    "Red",
                    "White",
                    "Yellow",
                ],
            )),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "Black" => Ok(Wrapper(AnsiColors::Black)),
            "Blue" => Ok(Wrapper(AnsiColors::Blue)),
            "BrightBlack" => Ok(Wrapper(AnsiColors::BrightBlack)),
            "BrightBlue" => Ok(Wrapper(AnsiColors::BrightBlue)),
            "BrightCyan" => Ok(Wrapper(AnsiColors::BrightCyan)),
            "BrightGreen" => Ok(Wrapper(AnsiColors::BrightGreen)),
            "BrightMagenta" => Ok(Wrapper(AnsiColors::BrightMagenta)),
            "BrightRed" => Ok(Wrapper(AnsiColors::BrightRed)),
            "BrightWhite" => Ok(Wrapper(AnsiColors::BrightWhite)),
            "BrightYellow" => Ok(Wrapper(AnsiColors::BrightYellow)),
            "Cyan" => Ok(Wrapper(AnsiColors::Cyan)),
            "Default" => Ok(Wrapper(AnsiColors::Default)),
            "Green" => Ok(Wrapper(AnsiColors::Green)),
            "Magenta" => Ok(Wrapper(AnsiColors::Magenta)),
            "Red" => Ok(Wrapper(AnsiColors::Red)),
            "White" => Ok(Wrapper(AnsiColors::White)),
            "Yellow" => Ok(Wrapper(AnsiColors::Yellow)),
            _ => Err(serde::de::Error::unknown_variant(
                v,
                &[
                    "Black",
                    "Blue",
                    "BrightBlack",
                    "BrightBlue",
                    "BrightCyan",
                    "BrightGreen",
                    "BrightMagenta",
                    "BrightRed",
                    "BrightWhite",
                    "BrightYellow",
                    "Cyan",
                    "Default",
                    "Green",
                    "Magenta",
                    "Red",
                    "White",
                    "Yellow",
                ],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for LoggerColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl Serialize for LoggerColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            AnsiColors::Black => serializer.serialize_str("Black"),
            AnsiColors::Red => serializer.serialize_str("Red"),
            AnsiColors::Green => serializer.serialize_str("Green"),
            AnsiColors::Yellow => serializer.serialize_str("Yellow"),
            AnsiColors::Blue => serializer.serialize_str("Blue"),
            AnsiColors::Magenta => serializer.serialize_str("Magenta"),
            AnsiColors::Cyan => serializer.serialize_str("Cyan"),
            AnsiColors::White => serializer.serialize_str("White"),
            AnsiColors::Default => serializer.serialize_str("Default"),
            AnsiColors::BrightBlack => serializer.serialize_str("BrightBlack"),
            AnsiColors::BrightRed => serializer.serialize_str("BrightRed"),
            AnsiColors::BrightGreen => serializer.serialize_str("BrightGreen"),
            AnsiColors::BrightYellow => serializer.serialize_str("BrightYellow"),
            AnsiColors::BrightBlue => serializer.serialize_str("BrightBlue"),
            AnsiColors::BrightMagenta => serializer.serialize_str("BrightMagenta"),
            AnsiColors::BrightCyan => serializer.serialize_str("BrightCyan"),
            AnsiColors::BrightWhite => serializer.serialize_str("BrightWhite"),
        }
    }
}
