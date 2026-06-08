use crate::bindings::{ApiCtx, RealTauriApi};
use crate::components::app_shell::AppShell;
use crate::state::app_state::{AppState, View};
use crate::state::playback_state::PlaybackState;
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::*;
use std::rc::Rc;

/// If launched as a picture-in-picture window, the URL carries `#pip=<script_id>`
/// (a hash fragment, so the Tauri asset resolver still serves index.html).
/// Returns that id so the app can boot straight into the prompter for it.
fn pip_script_id() -> Option<String> {
    let hash = web_sys::window()?.location().hash().ok()?;
    let h = hash.trim_start_matches('#');
    for pair in h.split('&') {
        let mut it = pair.splitn(2, '=');
        if it.next() == Some("pip") {
            return it.next().filter(|s| !s.is_empty()).map(|s| s.to_string());
        }
    }
    None
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
