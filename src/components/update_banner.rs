//! Self-update prompt.
//!
//! On mount, [`UpdateBanner`] asks the backend (through [`ApiCtx`]) whether a
//! newer release is available at the configured updater endpoint. If one is, it
//! renders a non-blocking banner with **Install** / **Dismiss**. Installing
//! downloads + applies the update and relaunches the app (handled natively by
//! `tauri-plugin-updater`).
//!
//! Behavior is deliberately conservative:
//! - up-to-date (`Ok(None)`) is silent — no toast, no banner;
//! - a failed *check* surfaces one error toast (beta builds want visibility);
//! - install is only ever user-initiated (never silent/auto-install).

use crate::bindings::ApiCtx;
use crate::state::toast::ToastState;
use leptos::*;

#[component]
pub fn UpdateBanner() -> impl IntoView {
    let api = use_context::<ApiCtx>().expect("AppApi not provided");
    let toast = use_context::<ToastState>().expect("ToastState not provided");

    // `Some(info)` once an update is available and not yet dismissed.
    let update = create_rw_signal::<Option<crate::bindings::tauri_api::UpdateInfo>>(None);
    let installing = create_rw_signal(false);

    // Auto-check once on mount. Silent when up to date; one toast on failure.
    {
        let api = api.clone();
        create_effect(move |_| {
            let api = api.clone();
            spawn_local(async move {
                match api.check_for_update().await {
                    Ok(Some(info)) => update.set(Some(info)),
                    Ok(None) => {}
                    Err(e) => toast.add_error(&format!("Update check failed: {e}")),
                }
            });
        });
    }

    let on_install: Callback<leptos::ev::MouseEvent> = Callback::new(move |_| {
        if installing.get_untracked() {
            return;
        }
        installing.set(true);
        let api = api.clone();
        spawn_local(async move {
            match api.install_update().await {
                // On success the native side downloads, installs, and relaunches;
                // the toast covers the brief pre-restart window (and tests).
                Ok(()) => toast.add_success("Installing update\u{2026} the app will restart."),
                Err(e) => {
                    toast.add_error(&format!("Update failed: {e}"));
                    installing.set(false);
                }
            }
        });
    });

    let on_dismiss: Callback<leptos::ev::MouseEvent> = Callback::new(move |_| update.set(None));

    view! {
        {move || update.get().map(|info| view! {
            <div
                class="update-banner"
                role="alert"
                style="
                    display: flex; align-items: center; gap: 12px;
                    padding: 8px 16px;
                    background: var(--bg-elevated); color: var(--text-main);
                    border-bottom: 1px solid var(--accent);
                    font-size: 13px;
                "
            >
                <span style="flex: 1;">
                    "Update available: v" {info.version}
                </span>
                <button
                    aria-label="Install update"
                    on:click=move |ev| on_install.call(ev)
                    prop:disabled=move || installing.get()
                    style="
                        background: var(--button-primary-bg); color: var(--button-primary-text);
                        border: none; border-radius: 6px; padding: 6px 14px; cursor: pointer;
                    "
                >
                    "Install"
                </button>
                <button
                    aria-label="Dismiss update"
                    on:click=move |ev| on_dismiss.call(ev)
                    style="
                        background: transparent; color: var(--button-ghost-text);
                        border: 1px solid var(--button-ghost-border); border-radius: 6px;
                        padding: 6px 14px; cursor: pointer;
                    "
                >
                    "Dismiss"
                </button>
            </div>
        })}
    }
}
