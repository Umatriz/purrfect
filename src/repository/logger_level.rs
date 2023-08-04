use serde::{de::Visitor, Deserialize, Serialize};

use crate::prelude::*;

pub type LoggerLevel = Wrapper<log::LevelFilter>;

impl Default for LoggerLevel {
    fn default() -> Self {
        Self(log::LevelFilter::Info)
    }
}

struct LoggerLevelVisitor;

impl<'de> Visitor<'de> for LoggerLevelVisitor {
    type Value = LoggerLevel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.as_str() {
            "Error" => Ok(Wrapper(log::LevelFilter::Error)),
            "Warn" => Ok(Wrapper(log::LevelFilter::Warn)),
            "Info" => Ok(Wrapper(log::LevelFilter::Info)),
            "Debug" => Ok(Wrapper(log::LevelFilter::Debug)),
            "Trace" => Ok(Wrapper(log::LevelFilter::Trace)),
            _ => Err(serde::de::Error::unknown_variant(
                &v,
                &["Error", "Warn", "Info", "Debug", "Trace"],
            )),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "Error" => Ok(Wrapper(log::LevelFilter::Error)),
            "Warn" => Ok(Wrapper(log::LevelFilter::Warn)),
            "Info" => Ok(Wrapper(log::LevelFilter::Info)),
            "Debug" => Ok(Wrapper(log::LevelFilter::Debug)),
            "Trace" => Ok(Wrapper(log::LevelFilter::Trace)),
            _ => Err(serde::de::Error::unknown_variant(
                v,
                &["Error", "Warn", "Info", "Debug", "Trace"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for LoggerLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(LoggerLevelVisitor)
    }
}

impl Serialize for LoggerLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            log::LevelFilter::Off => serializer.serialize_str("Off"),
            log::LevelFilter::Error => serializer.serialize_str("Error"),
            log::LevelFilter::Warn => serializer.serialize_str("Warn"),
            log::LevelFilter::Info => serializer.serialize_str("Info"),
            log::LevelFilter::Debug => serializer.serialize_str("Debug"),
            log::LevelFilter::Trace => serializer.serialize_str("Trace"),
        }
    }
}
