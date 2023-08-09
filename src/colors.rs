use serde::{Deserialize, Serialize};

use crate::repository::color::LoggerColor;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Colors {
    pub(super) background: LoggerColor,
    pub(super) color: LoggerColor,
}
