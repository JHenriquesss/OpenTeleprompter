use leptos::*;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

fn next_toast_id() -> u32 {
    static ID: AtomicU32 = AtomicU32::new(1);
    ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub enum ToastLevel {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastLevel {
    pub fn css_class(&self) -> &'static str {
        match self {
            ToastLevel::Success => "toast-success",
            ToastLevel::Error => "toast-error",
            ToastLevel::Warning => "toast-warning",
            ToastLevel::Info => "toast-info",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToastLevel::Success => "\u{2713}",
            ToastLevel::Error => "\u{2717}",
            ToastLevel::Warning => "\u{26a0}",
            ToastLevel::Info => "\u{2139}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub id: u32,
    pub message: String,
    pub level: ToastLevel,
}

#[derive(Debug, Clone, Copy)]
pub struct ToastState {
    pub messages: RwSignal<Vec<ToastMessage>>,
}

impl ToastState {
    pub fn new() -> Self {
        Self {
            messages: RwSignal::new(Vec::new()),
        }
    }

    pub fn add_success(&self, msg: &str) {
        self.add(msg, ToastLevel::Success);
    }

    pub fn add_error(&self, msg: &str) {
        self.add(msg, ToastLevel::Error);
    }

    pub fn add_warning(&self, msg: &str) {
        self.add(msg, ToastLevel::Warning);
    }

    pub fn add_info(&self, msg: &str) {
        self.add(msg, ToastLevel::Info);
    }

    fn add(&self, msg: &str, level: ToastLevel) {
        let id = next_toast_id();
        let toast = ToastMessage {
            id,
            message: msg.to_string(),
            level,
        };
        self.messages.update(|m| m.push(toast));

        let msgs = self.messages;
        let window = web_sys::window().unwrap();
        let closure = Closure::once(move || {
            msgs.update(|m| m.retain(|t| t.id != id));
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                4000,
            )
            .unwrap();
        closure.forget();
    }
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let toast = expect_context::<ToastState>();

    view! {
        <div style="
            position: fixed; bottom: 20px; right: 20px;
            z-index: 1000; display: flex; flex-direction: column;
            gap: 8px; pointer-events: none;
        ">
            <For
                each=move || toast.messages.get()
                key=|t| t.id
                let:t
            >
                <div
                    style="
                        padding: 10px 16px; border-radius: 8px;
                        font-size: 13px; line-height: 1.4;
                        pointer-events: auto;
                        display: flex; align-items: center; gap: 8px;
                        animation: toastIn 0.2s ease-out;
                        max-width: 320px; word-break: break-word;
                    "
                    class={t.level.css_class()}
                >
                    <span style="font-weight: bold; flex-shrink: 0;">{t.level.icon()}</span>
                    <span>{t.message}</span>
                    <button
                        on:click=move |_| toast.messages.update(|m| m.retain(|x| x.id != t.id))
                        style="
                            margin-left: auto; background: none; border: none;
                            color: inherit; cursor: pointer; font-size: 14px;
                            padding: 0 0 0 8px; opacity: 0.7; flex-shrink: 0;
                        "
                    >
                        "\u{2715}"
                    </button>
                </div>
            </For>
        </div>
        <style>
            {r#"
            .toast-success { background: #1b5e20; color: #fff; border: 1px solid #2e7d32; }
            .toast-error { background: #b71c1c; color: #fff; border: 1px solid #c62828; }
            .toast-warning { background: #e65100; color: #fff; border: 1px solid #ef6c00; }
            .toast-info { background: #0d47a1; color: #fff; border: 1px solid #1565c0; }
            @keyframes toastIn {
                from { opacity: 0; transform: translateY(8px); }
                to { opacity: 1; transform: translateY(0); }
            }
            "#}
        </style>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_add_success_toast() {
        let state = ToastState::new();
        state.add_success("Saved OK");
        let msgs = state.messages.get();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].message, "Saved OK");
        assert!(matches!(msgs[0].level, ToastLevel::Success));
    }

    #[wasm_bindgen_test]
    fn test_add_error_toast() {
        let state = ToastState::new();
        state.add_error("Failed");
        let msgs = state.messages.get();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].message, "Failed");
        assert!(matches!(msgs[0].level, ToastLevel::Error));
    }

    #[wasm_bindgen_test]
    fn test_add_warning_toast() {
        let state = ToastState::new();
        state.add_warning("Be careful");
        let msgs = state.messages.get();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].message, "Be careful");
        assert!(matches!(msgs[0].level, ToastLevel::Warning));
    }

    #[wasm_bindgen_test]
    fn test_add_info_toast() {
        let state = ToastState::new();
        state.add_info("+5s");
        let msgs = state.messages.get();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].message, "+5s");
        assert!(matches!(msgs[0].level, ToastLevel::Info));
    }

    #[wasm_bindgen_test]
    fn test_dismiss_toast() {
        let state = ToastState::new();
        state.add_success("Dismiss me");
        let id = state.messages.get()[0].id;
        state.messages.update(|m| m.retain(|t| t.id != id));
        assert!(state.messages.get().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_multiple_toasts_have_unique_ids() {
        let state = ToastState::new();
        state.add_success("first");
        state.add_error("second");
        state.add_info("third");
        let msgs = state.messages.get();
        assert_eq!(msgs.len(), 3);
        let ids: Vec<u32> = msgs.iter().map(|m| m.id).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 3, "toast IDs must be unique");
    }

    #[wasm_bindgen_test]
    fn test_toast_level_css_class() {
        assert_eq!(ToastLevel::Success.css_class(), "toast-success");
        assert_eq!(ToastLevel::Error.css_class(), "toast-error");
        assert_eq!(ToastLevel::Warning.css_class(), "toast-warning");
        assert_eq!(ToastLevel::Info.css_class(), "toast-info");
    }

    #[wasm_bindgen_test]
    fn test_toast_level_icon() {
        assert_eq!(ToastLevel::Success.icon(), "\u{2713}");
        assert_eq!(ToastLevel::Error.icon(), "\u{2717}");
        assert_eq!(ToastLevel::Warning.icon(), "\u{26a0}");
        assert_eq!(ToastLevel::Info.icon(), "\u{2139}");
    }
}
