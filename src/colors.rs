use serde::{Deserialize, Serialize};

use crate::repository::color::LoggerColor;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct LevelColors {
    pub(super) background: LoggerColor,
    pub(super) color: LoggerColor,
}
