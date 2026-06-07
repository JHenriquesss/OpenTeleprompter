use serde::{Deserialize, Serialize};

fn default_countdown() -> u32 {
    3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub font_size: f64,
    pub line_height: f64,
    pub text_width: f64,
    pub scroll_speed: f64,
    pub mirror_mode: bool,
    pub theme: Theme,
    #[serde(default = "default_countdown")]
    pub countdown_seconds: u32,
    #[serde(default)]
    pub mirror_vertical: bool,
    #[serde(default)]
    pub reading_guide_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            font_size: 32.0,
            line_height: 1.8,
            text_width: 60.0,
            scroll_speed: 1.0,
            mirror_mode: false,
            theme: Theme::Dark,
            countdown_seconds: 3,
            mirror_vertical: false,
            reading_guide_enabled: false,
        }
    }
}
