use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPlaybackState {
    pub script_id: String,
    pub scroll_offset_px: f64,
    pub speed_multiplier: f64,
    pub font_size: Option<f64>,
    pub line_height: Option<f64>,
    pub mirror_mode: Option<bool>,
    pub mirror_vertical: Option<bool>,
    pub updated_at: String,
}
