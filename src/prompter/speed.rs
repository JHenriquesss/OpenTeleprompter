pub const MIN_SPEED: f64 = 0.25;
pub const MAX_SPEED: f64 = 5.0;

pub fn validate_speed(val: f64) -> Option<f64> {
    if val.is_finite() && val >= MIN_SPEED && val <= MAX_SPEED {
        Some((val * 100.0).round() / 100.0)
    } else {
        None
    }
}

pub fn speed_label(speed: f64) -> &'static str {
    if speed <= 0.5 {
        "Slow"
    } else if speed <= 1.0 {
        "Normal"
    } else if speed <= 1.5 {
        "Fast"
    } else {
        "Very Fast"
    }
}

pub fn speed_presets() -> Vec<(&'static str, f64)> {
    vec![
        ("Slow", 0.5),
        ("Normal", 1.0),
        ("Fast", 1.5),
        ("Very Fast", 2.0),
    ]
}

pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn estimated_reading_seconds(text: &str, wpm: f64) -> u64 {
    let wc = word_count(text) as f64;
    if wc == 0.0 {
        return 0;
    }
    ((wc / wpm) * 60.0).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_validate_speed() {
        assert_eq!(validate_speed(1.0), Some(1.0));
        assert_eq!(validate_speed(0.25), Some(0.25));
        assert_eq!(validate_speed(5.0), Some(5.0));
        assert_eq!(validate_speed(0.0), None);
        assert_eq!(validate_speed(5.01), None);
        assert_eq!(validate_speed(f64::NAN), None);
        assert_eq!(validate_speed(f64::INFINITY), None);
        assert_eq!(validate_speed(-0.1), None);
    }

    #[wasm_bindgen_test]
    fn test_speed_label() {
        assert_eq!(speed_label(0.25), "Slow");
        assert_eq!(speed_label(0.5), "Slow");
        assert_eq!(speed_label(0.75), "Normal");
        assert_eq!(speed_label(1.0), "Normal");
        assert_eq!(speed_label(1.25), "Fast");
        assert_eq!(speed_label(1.5), "Fast");
        assert_eq!(speed_label(2.0), "Very Fast");
        assert_eq!(speed_label(10.0), "Very Fast");
    }

    #[wasm_bindgen_test]
    fn test_speed_presets() {
        let presets = speed_presets();
        assert_eq!(presets.len(), 4);
        assert_eq!(presets[0], ("Slow", 0.5));
        assert_eq!(presets[1], ("Normal", 1.0));
        assert_eq!(presets[2], ("Fast", 1.5));
        assert_eq!(presets[3], ("Very Fast", 2.0));
    }

    #[wasm_bindgen_test]
    fn test_word_count() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("hello"), 1);
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("  spaced  out  "), 2);
        assert_eq!(word_count("one two three four five"), 5);
    }

    #[wasm_bindgen_test]
    fn test_estimated_reading_seconds() {
        assert_eq!(estimated_reading_seconds("", 130.0), 0);
        assert_eq!(estimated_reading_seconds("hello world", 130.0), 1);
        assert_eq!(estimated_reading_seconds("word ", 130.0), 1);
        let text_130_words = (0..130)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(estimated_reading_seconds(&text_130_words, 130.0), 60);
        assert_eq!(estimated_reading_seconds("hello", 1000.0), 1);
    }
}
