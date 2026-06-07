use leptos::{RwSignal, SignalUpdate};

#[derive(Debug, Clone, Copy)]
pub struct UiState {
    pub font_size: RwSignal<f64>,
    pub line_height: RwSignal<f64>,
    pub text_width: RwSignal<f64>,
    pub mirror_mode: RwSignal<bool>,
    pub mirror_vertical: RwSignal<bool>,
    pub countdown_seconds: RwSignal<u32>,
    pub reading_guide: RwSignal<bool>,
    pub show_shortcut_help: RwSignal<bool>,
    pub theme: RwSignal<String>,
    pub rehearse_mode: RwSignal<bool>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            font_size: RwSignal::new(32.0),
            line_height: RwSignal::new(1.8),
            text_width: RwSignal::new(60.0),
            mirror_mode: RwSignal::new(false),
            mirror_vertical: RwSignal::new(false),
            countdown_seconds: RwSignal::new(3),
            reading_guide: RwSignal::new(false),
            show_shortcut_help: RwSignal::new(false),
            theme: RwSignal::new("Dark".to_string()),
            rehearse_mode: RwSignal::new(false),
        }
    }

    pub fn increase_font_size(&self) {
        self.font_size.update(|s| *s = (*s + 2.0).min(72.0));
    }

    pub fn decrease_font_size(&self) {
        self.font_size.update(|s| *s = (*s - 2.0).max(12.0));
    }

    pub fn toggle_mirror(&self) {
        self.mirror_mode.update(|m| *m = !*m);
    }

    pub fn toggle_mirror_vertical(&self) {
        self.mirror_vertical.update(|m| *m = !*m);
    }

    pub fn toggle_reading_guide(&self) {
        self.reading_guide.update(|r| *r = !*r);
    }

    pub fn toggle_shortcut_help(&self) {
        self.show_shortcut_help.update(|h| *h = !*h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::{SignalGet, SignalSet};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_ui_initial_state() {
        let ui = UiState::new();
        assert!((ui.font_size.get() - 32.0).abs() < 0.001);
        assert!((ui.line_height.get() - 1.8).abs() < 0.001);
        assert_eq!(ui.theme.get(), "Dark");
        assert!(!ui.reading_guide.get());
        assert!(!ui.show_shortcut_help.get());
        assert!(!ui.mirror_mode.get());
    }

    #[wasm_bindgen_test]
    fn test_increase_font_size() {
        let ui = UiState::new();
        ui.increase_font_size();
        assert!((ui.font_size.get() - 34.0).abs() < 0.001);
    }

    #[wasm_bindgen_test]
    fn test_decrease_font_size() {
        let ui = UiState::new();
        ui.decrease_font_size();
        assert!((ui.font_size.get() - 30.0).abs() < 0.001);
    }

    #[wasm_bindgen_test]
    fn test_increase_font_size_clamps_max() {
        let ui = UiState::new();
        ui.font_size.set(72.0);
        ui.increase_font_size();
        assert!((ui.font_size.get() - 72.0).abs() < 0.001);
    }

    #[wasm_bindgen_test]
    fn test_decrease_font_size_clamps_min() {
        let ui = UiState::new();
        ui.font_size.set(12.0);
        ui.decrease_font_size();
        assert!((ui.font_size.get() - 12.0).abs() < 0.001);
    }

    #[wasm_bindgen_test]
    fn test_toggle_mirror() {
        let ui = UiState::new();
        ui.toggle_mirror();
        assert!(ui.mirror_mode.get());
        ui.toggle_mirror();
        assert!(!ui.mirror_mode.get());
    }

    #[wasm_bindgen_test]
    fn test_toggle_mirror_vertical() {
        let ui = UiState::new();
        ui.toggle_mirror_vertical();
        assert!(ui.mirror_vertical.get());
    }

    #[wasm_bindgen_test]
    fn test_toggle_reading_guide() {
        let ui = UiState::new();
        ui.toggle_reading_guide();
        assert!(ui.reading_guide.get());
        ui.toggle_reading_guide();
        assert!(!ui.reading_guide.get());
    }

    #[wasm_bindgen_test]
    fn test_toggle_shortcut_help() {
        let ui = UiState::new();
        ui.toggle_shortcut_help();
        assert!(ui.show_shortcut_help.get());
        ui.toggle_shortcut_help();
        assert!(!ui.show_shortcut_help.get());
    }
}
