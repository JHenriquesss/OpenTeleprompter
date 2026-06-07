use crate::state::playback_state::PlaybackState;
use leptos::*;

#[component]
pub fn Toolbar() -> impl IntoView {
    let playback = use_context::<PlaybackState>().expect("PlaybackState not provided");

    view! {
        <div style="
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 8px 16px;
            background: #16213e;
            border-bottom: 1px solid #0f3460;
        ">
            <button
                on:click=move |_| {
                    if playback.is_playing.get() {
                        playback.is_playing.set(false)
                    } else {
                        playback.is_playing.set(true)
                    }
                }
                style="
                    padding: 6px 14px;
                    border: 1px solid #555;
                    border-radius: 4px;
                    background: transparent;
                    color: #e0e0e0;
                    cursor: pointer;
                    font-size: 14px;
                "
            >
                {move || if playback.is_playing.get() { "⏸" } else { "▶" }}
            </button>

            <button
                on:click=move |_| playback.restart()
                style="
                    padding: 6px 10px;
                    border: 1px solid #555;
                    border-radius: 4px;
                    background: transparent;
                    color: #e0e0e0;
                    cursor: pointer;
                    font-size: 12px;
                "
            >
                {"↺"}
            </button>

            <div style="flex: 1;"></div>

            <div style="display: flex; align-items: center; gap: 4px; font-size: 12px; color: #888;">
                <span>Speed:</span>
                <span>{move || format!("{:.1}x", playback.speed.get())}</span>
            </div>
        </div>
    }
}
