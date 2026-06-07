use crate::components::app_shell::AppShell;
use crate::state::app_state::AppState;
use crate::state::playback_state::PlaybackState;
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    let playback_state = PlaybackState::new();
    let ui_state = UiState::new();
    let toast_state = ToastState::new();

    provide_context(app_state);
    provide_context(playback_state);
    provide_context(ui_state);
    provide_context(toast_state);

    view! {
        <AppShell />
    }
}
