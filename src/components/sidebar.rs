use crate::bindings::ApiCtx;
use crate::state::app_state::{AppState, View};
use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    let api = use_context::<ApiCtx>().expect("AppApi not provided");
    let (version, set_version) = create_signal("0.10.0".to_string());

    create_effect(move |_| {
        let api = api.clone();
        spawn_local(async move {
            if let Ok(v) = api.get_app_version().await {
                set_version.set(v);
            }
        });
    });

    view! {
        <div style="
            width: 60px;
            background: var(--bg-panel);
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 16px 0;
            gap: 8px;
            border-right: 1px solid var(--card-border);
        ">
            <div style="
                font-size: 20px;
                font-weight: bold;
                color: var(--accent);
                margin-bottom: 24px;
            ">OP</div>

            <button
                on:click=move |_| app_state.view.set(View::Library)
                title="Script Library"
                style="
                    width: 40px; height: 40px;
                    border: none; border-radius: 8px;
                    cursor: pointer;
                    font-size: 18px;
                    background: transparent;
                    color: var(--button-ghost-text);
                    transition: all 0.2s;
                "
            >
                {"📋"}
            </button>

            <button
                on:click=move |_| app_state.view.set(View::Settings)
                title="Settings"
                style="
                    width: 40px; height: 40px;
                    border: none; border-radius: 8px;
                    cursor: pointer;
                    font-size: 18px;
                    background: transparent;
                    color: var(--button-ghost-text);
                    transition: all 0.2s;
                "
            >
                {"⚙️"}
            </button>

            <div style="flex: 1;"></div>

            <div style="font-size: 10px; color: var(--text-dim); writing-mode: vertical-rl; text-orientation: mixed;">
                "v" {move || version.get()}
            </div>
        </div>
    }
}
