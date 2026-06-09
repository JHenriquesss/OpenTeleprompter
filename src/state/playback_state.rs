use leptos::{RwSignal, SignalSet, SignalUpdate};

#[derive(Debug, Clone, Copy)]
pub struct PlaybackState {
    pub is_playing: RwSignal<bool>,
    pub scroll_y: RwSignal<f64>,
    pub speed: RwSignal<f64>,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            is_playing: RwSignal::new(false),
            scroll_y: RwSignal::new(0.0),
            speed: RwSignal::new(1.0),
        }
    }

    pub fn toggle_play(&self) {
        self.is_playing.update(|p| *p = !*p);
    }

    pub fn restart(&self) {
        self.scroll_y.set(0.0);
        self.is_playing.set(true);
    }

    pub fn increase_speed(&self) {
        // Fine 0.05 steps for precise control; round to 2 dp to avoid float drift.
        self.speed
            .update(|s| *s = (((*s + 0.05) * 100.0).round() / 100.0).min(10.0));
    }

    pub fn decrease_speed(&self) {
        self.speed
            .update(|s| *s = (((*s - 0.05) * 100.0).round() / 100.0).max(0.25));
    }

    pub fn set_speed(&self, val: f64) {
        self.speed.set(val);
    }

    pub fn jump_forward(&self) {
        self.scroll_y.update(|y| *y = (*y + 200.0).min(100000.0));
    }

    pub fn jump_backward(&self) {
        self.scroll_y.update(|y| *y = (*y - 200.0).max(0.0));
    }

    pub fn jump_big_forward(&self) {
        self.scroll_y.update(|y| *y = (*y + 800.0).min(100000.0));
    }

    pub fn jump_big_backward(&self) {
        self.scroll_y.update(|y| *y = (*y - 800.0).max(0.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::{SignalGet, SignalSet};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_playback_initial_state() {
        let pb = PlaybackState::new();
        assert!(!pb.is_playing.get());
        assert_eq!(pb.scroll_y.get(), 0.0);
        assert_eq!(pb.speed.get(), 1.0);
    }

    #[wasm_bindgen_test]
    fn test_toggle_play() {
        let pb = PlaybackState::new();
        pb.toggle_play();
        assert!(pb.is_playing.get());
        pb.toggle_play();
        assert!(!pb.is_playing.get());
    }

    #[wasm_bindgen_test]
    fn test_restart() {
        let pb = PlaybackState::new();
        pb.scroll_y.set(500.0);
        pb.is_playing.set(false);
        pb.restart();
        assert_eq!(pb.scroll_y.get(), 0.0);
        assert!(pb.is_playing.get());
    }

    #[wasm_bindgen_test]
    fn test_increase_speed() {
        let pb = PlaybackState::new();
        pb.increase_speed();
        assert_eq!(pb.speed.get(), 1.05);
    }

    #[wasm_bindgen_test]
    fn test_decrease_speed() {
        let pb = PlaybackState::new();
        pb.decrease_speed();
        assert_eq!(pb.speed.get(), 0.95);
    }

    #[wasm_bindgen_test]
    fn test_set_speed() {
        let pb = PlaybackState::new();
        pb.set_speed(3.5);
        assert!((pb.speed.get() - 3.5).abs() < 0.001);
    }

    #[wasm_bindgen_test]
    fn test_jump_forward() {
        let pb = PlaybackState::new();
        pb.jump_forward();
        assert_eq!(pb.scroll_y.get(), 200.0);
    }

    #[wasm_bindgen_test]
    fn test_jump_backward() {
        let pb = PlaybackState::new();
        pb.scroll_y.set(500.0);
        pb.jump_backward();
        assert_eq!(pb.scroll_y.get(), 300.0);
    }

    #[wasm_bindgen_test]
    fn test_jump_big_forward() {
        let pb = PlaybackState::new();
        pb.jump_big_forward();
        assert_eq!(pb.scroll_y.get(), 800.0);
    }

    #[wasm_bindgen_test]
    fn test_jump_big_backward() {
        let pb = PlaybackState::new();
        pb.scroll_y.set(2000.0);
        pb.jump_big_backward();
        assert_eq!(pb.scroll_y.get(), 1200.0);
    }

    #[wasm_bindgen_test]
    fn test_jump_forward_clamps_max() {
        let pb = PlaybackState::new();
        pb.scroll_y.set(100000.0);
        pb.jump_forward();
        assert_eq!(pb.scroll_y.get(), 100000.0);
    }

    #[wasm_bindgen_test]
    fn test_jump_backward_clamps_min() {
        let pb = PlaybackState::new();
        pb.jump_backward();
        assert_eq!(pb.scroll_y.get(), 0.0);
    }

    #[wasm_bindgen_test]
    fn test_increase_speed_clamps_max() {
        let pb = PlaybackState::new();
        pb.speed.set(10.0);
        pb.increase_speed();
        assert_eq!(pb.speed.get(), 10.0);
    }

    #[wasm_bindgen_test]
    fn test_decrease_speed_clamps_min() {
        let pb = PlaybackState::new();
        pb.speed.set(0.25);
        pb.decrease_speed();
        assert_eq!(pb.speed.get(), 0.25);
    }
}
