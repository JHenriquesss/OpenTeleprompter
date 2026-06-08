use crate::bindings::{ApiCtx, RealTauriApi};
use crate::components::app_shell::AppShell;
use crate::state::app_state::{AppState, View};
use crate::state::playback_state::PlaybackState;
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::*;
use std::rc::Rc;

/// If launched as a picture-in-picture window, the backend injects
/// `window.__PIP_SCRIPT_ID` via an init script. Returns that id so the app can
/// boot straight into the prompter for it.
fn pip_script_id() -> Option<String> {
    let win = web_sys::window()?;
    let v = js_sys::Reflect::get(&win, &wasm_bindgen::JsValue::from_str("__PIP_SCRIPT_ID")).ok()?;
    v.as_string().filter(|s| !s.is_empty())
}

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    let playback_state = PlaybackState::new();
    let ui_state = UiState::new();
    let toast_state = ToastState::new();

    // Picture-in-picture window: boot directly into the prompter for the script.
    if let Some(id) = pip_script_id() {
        app_state.selected_script_id.set(Some(id));
        app_state.view.set(View::Prompter);
    }

    let api: ApiCtx = Rc::new(RealTauriApi);

    provide_context(api);
    provide_context(app_state);
    provide_context(playback_state);
    provide_context(ui_state);
    provide_context(toast_state);

    view! {
        <AppShell />
    }
}
