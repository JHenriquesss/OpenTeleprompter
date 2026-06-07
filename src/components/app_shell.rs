use crate::bindings::ApiCtx;
use crate::components::prompter_view::PrompterView;
use crate::components::script_editor::ScriptEditor;
use crate::components::script_library::ScriptLibrary;
use crate::components::settings_panel::SettingsPanel;
use crate::components::sidebar::Sidebar;
use crate::components::update_banner::UpdateBanner;
use crate::state::app_state::View;
use crate::state::toast::{ToastContainer, ToastState};
use crate::state::ui_state::UiState;
use leptos::*;

#[component]
pub fn AppShell() -> impl IntoView {
    let app_state =
        use_context::<crate::state::app_state::AppState>().expect("AppState not provided");
    let ui = use_context::<UiState>().expect("UiState not provided");
    let api = use_context::<ApiCtx>().expect("AppApi not provided");
    let toast = use_context::<ToastState>().expect("ToastState not provided");

    let current_view = move || app_state.view.get();

    create_effect(move |_| {
        let api = api.clone();
        spawn_local(async move {
            if let Ok(s) = api.get_settings().await {
                ui.theme.set(s.theme);
            }
        });
    });

    // One-time hint when the window is hidden to the tray (Phase 16). Tauri's
    // `event.once` fires at most once per app run.
    create_effect(move |_| {
        crate::bindings::tauri_api::on_close_to_tray_once(move || {
            toast.add_info(
                "OpenPrompter is still running in the system tray. Use the tray icon to quit.",
            );
        });
    });

    view! {
        <div
            class={move || format!("app-shell theme-{}", ui.theme.get().to_lowercase())}
            style="display: flex; height: 100vh; width: 100vw; background: var(--bg-main); color: var(--text-main);"
        >
            <style>{GLOBAL_CSS}</style>
            <ToastContainer />
            {move || match current_view() {
                View::Prompter => {
                    view! { <PrompterView /> }.into_view()
                }
                _ => {
                    view! {
                        <div style="display: flex; flex-direction: column; flex: 1; overflow: hidden;">
                            <UpdateBanner />
                            <div style="display: flex; flex: 1; overflow: hidden;">
                                <Sidebar />
                                <div style="flex: 1; overflow: hidden;">
                                    {move || match current_view() {
                                        View::Library => {
                                            view! { <ScriptLibrary /> }.into_view()
                                        }
                                        View::Editor => {
                                            view! { <ScriptEditor /> }.into_view()
                                        }
                                        View::Settings => {
                                            view! { <SettingsPanel /> }.into_view()
                                        }
                                        _ => {
                                            view! { <ScriptLibrary /> }.into_view()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

const GLOBAL_CSS: &str = "\
.theme-dark, .theme-light {\
  --bg-main: #1a1a2e;\
  --bg-panel: #16213e;\
  --bg-elevated: #0f3460;\
  --bg-overlay: rgba(0,0,0,0.6);\
  --bg-code: #16213e;\
  --text-main: #e0e0e0;\
  --text-muted: #aaa;\
  --text-dim: #555;\
  --text-muted2: #666;\
  --text-loading: #888;\
  --border-color: #333;\
  --border-light: #555;\
  --accent: #e94560;\
  --danger-bg: #4a0e0e;\
  --danger-text: #e94560;\
  --success-text: #5cb85c;\
  --warning-text: #f0ad4e;\
  --button-primary-bg: #e94560;\
  --button-primary-text: #fff;\
  --button-secondary-bg: #0f3460;\
  --button-secondary-text: #fff;\
  --button-ghost-text: #e0e0e0;\
  --button-ghost-border: #555;\
  --input-bg: #16213e;\
  --input-text: #e0e0e0;\
  --input-border: #333;\
  --scrollbar-thumb: #555;\
  --scrollbar-hover: #777;\
  --card-bg: #16213e;\
  --card-border: #0f3460;\
  --card-text: #e0e0e0;\
}\
.theme-dark {\
  --bg-main: #1a1a2e;\
  --bg-panel: #16213e;\
  --bg-elevated: #0f3460;\
  --bg-overlay: rgba(0,0,0,0.6);\
  --bg-code: #16213e;\
  --text-main: #e0e0e0;\
  --text-muted: #aaa;\
  --text-dim: #555;\
  --text-muted2: #666;\
  --text-loading: #888;\
  --border-color: #333;\
  --border-light: #555;\
  --accent: #e94560;\
  --danger-bg: #4a0e0e;\
  --danger-text: #e94560;\
  --success-text: #5cb85c;\
  --warning-text: #f0ad4e;\
  --button-primary-bg: #e94560;\
  --button-primary-text: #fff;\
  --button-secondary-bg: #0f3460;\
  --button-secondary-text: #fff;\
  --button-ghost-text: #e0e0e0;\
  --button-ghost-border: #555;\
  --input-bg: #16213e;\
  --input-text: #e0e0e0;\
  --input-border: #333;\
  --scrollbar-thumb: #555;\
  --scrollbar-hover: #777;\
  --card-bg: #16213e;\
  --card-border: #0f3460;\
  --card-text: #e0e0e0;\
}\
.theme-light {\
  --bg-main: #ffffff;\
  --bg-panel: #f5f5f5;\
  --bg-elevated: #e8e8e8;\
  --bg-overlay: rgba(0,0,0,0.4);\
  --bg-code: #f5f5f5;\
  --text-main: #1a1a2e;\
  --text-muted: #555;\
  --text-dim: #888;\
  --text-muted2: #999;\
  --text-loading: #888;\
  --border-color: #ddd;\
  --border-light: #ccc;\
  --accent: #e94560;\
  --danger-bg: #fde8e8;\
  --danger-text: #d32f2f;\
  --success-text: #2e7d32;\
  --warning-text: #e65100;\
  --button-primary-bg: #e94560;\
  --button-primary-text: #fff;\
  --button-secondary-bg: #16213e;\
  --button-secondary-text: #fff;\
  --button-ghost-text: #333;\
  --button-ghost-border: #ccc;\
  --input-bg: #ffffff;\
  --input-text: #1a1a2e;\
  --input-border: #ccc;\
  --scrollbar-thumb: #ccc;\
  --scrollbar-hover: #aaa;\
  --card-bg: #ffffff;\
  --card-border: #ddd;\
  --card-text: #1a1a2e;\
}\
::-webkit-scrollbar { width: 8px; height: 8px; }\
::-webkit-scrollbar-track { background: transparent; }\
::-webkit-scrollbar-thumb { background: var(--scrollbar-thumb); border-radius: 4px; }\
::-webkit-scrollbar-thumb:hover { background: var(--scrollbar-hover); }\
::-webkit-scrollbar-corner { background: transparent; }\
";
