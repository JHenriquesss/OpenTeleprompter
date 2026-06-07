use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub scroll_y: f64,
    pub speed: f64,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            scroll_y: 0.0,
            speed: 1.0,
        }
    }
}
