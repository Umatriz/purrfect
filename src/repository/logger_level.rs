use serde::{de::Visitor, Deserialize, Serialize};

use crate::prelude::Wrapper;

pub type LoggerLevel = Wrapper<log::Level>;

impl Default for LoggerLevel {
    fn default() -> Self {
        Self(log::Level::Info)
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
            "Error" => Ok(Wrapper(log::Level::Error)),
            "Warn" => Ok(Wrapper(log::Level::Warn)),
            "Info" => Ok(Wrapper(log::Level::Info)),
            "Debug" => Ok(Wrapper(log::Level::Debug)),
            "Trace" => Ok(Wrapper(log::Level::Trace)),
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
            "Error" => Ok(Wrapper(log::Level::Error)),
            "Warn" => Ok(Wrapper(log::Level::Warn)),
            "Info" => Ok(Wrapper(log::Level::Info)),
            "Debug" => Ok(Wrapper(log::Level::Debug)),
            "Trace" => Ok(Wrapper(log::Level::Trace)),
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
            log::Level::Error => serializer.serialize_str("Error"),
            log::Level::Warn => serializer.serialize_str("Warn"),
            log::Level::Info => serializer.serialize_str("Info"),
            log::Level::Debug => serializer.serialize_str("Debug"),
            log::Level::Trace => serializer.serialize_str("Trace"),
        }
    }
}
