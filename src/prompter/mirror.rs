use leptos::{Signal, SignalGet};

pub fn mirror_transform(enabled: Signal<bool>) -> &'static str {
    if enabled.get() {
        "scaleX(-1)"
    } else {
        "none"
    }
}

pub fn mirror_transform_combined(horizontal: Signal<bool>, vertical: Signal<bool>) -> String {
    match (horizontal.get(), vertical.get()) {
        (true, true) => "scaleX(-1) scaleY(-1)".to_string(),
        (true, false) => "scaleX(-1)".to_string(),
        (false, true) => "scaleY(-1)".to_string(),
        (false, false) => "none".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::{create_rw_signal, Signal};
    use wasm_bindgen_test::wasm_bindgen_test;

    fn sig(val: bool) -> Signal<bool> {
        Signal::from(create_rw_signal(val))
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_enabled() {
        assert_eq!(mirror_transform(sig(true)), "scaleX(-1)");
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_disabled() {
        assert_eq!(mirror_transform(sig(false)), "none");
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_combined_none() {
        assert_eq!(mirror_transform_combined(sig(false), sig(false)), "none");
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_combined_horizontal() {
        assert_eq!(
            mirror_transform_combined(sig(true), sig(false)),
            "scaleX(-1)"
        );
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_combined_vertical() {
        assert_eq!(
            mirror_transform_combined(sig(false), sig(true)),
            "scaleY(-1)"
        );
    }

    #[wasm_bindgen_test]
    fn test_mirror_transform_combined_both() {
        assert_eq!(
            mirror_transform_combined(sig(true), sig(true)),
            "scaleX(-1) scaleY(-1)"
        );
    }
}
