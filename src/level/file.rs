use std::ops::Range;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LevelData {
    pub name: String,
    pub columns: [Column; 6],
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Column {
    pub speed: f32,
    pub notes: Vec<Range<f32>>,
}
