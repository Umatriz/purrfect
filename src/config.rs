use serde::{Deserialize, Serialize};

use crate::colors::Color;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub level: LevelColors,
}

#[derive(Deserialize, Serialize)]
pub struct LevelColors {
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}
