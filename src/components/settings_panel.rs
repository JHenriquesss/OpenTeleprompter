use crate::bindings::tauri_api::AppSettingsData;
use crate::bindings::ApiCtx;
use crate::state::playback_state::PlaybackState;
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::*;

#[component]
pub fn SettingsPanel() -> impl IntoView {
    let ui = use_context::<UiState>().expect("UiState not provided");
    let playback = use_context::<PlaybackState>().expect("PlaybackState not provided");
    let toast = expect_context::<ToastState>();
    let api = use_context::<ApiCtx>().expect("AppApi not provided");
    let (settings_error, set_settings_error) = create_signal::<Option<String>>(None);
    let (settings_loaded, set_settings_loaded) = create_signal(false);
    let (app_version, set_app_version) = create_signal(String::new());

    create_effect({
        let api = api.clone();
        move |_| {
            let api = api.clone();
            spawn_local(async move {
                match api.get_settings().await {
                    Ok(s) => {
                        ui.font_size.set(s.font_size);
                        ui.line_height.set(s.line_height);
                        ui.text_width.set(s.text_width);
                        ui.mirror_mode.set(s.mirror_mode);
                        ui.countdown_seconds.set(s.countdown_seconds);
                        ui.mirror_vertical.set(s.mirror_vertical);
                        ui.reading_guide.set(s.reading_guide_enabled);
                        ui.theme.set(s.theme.clone());
                        playback.speed.set(s.scroll_speed);
                        set_settings_loaded.set(true);
                    }
                    Err(e) => {
                        set_settings_error.set(Some(e));
                        set_settings_loaded.set(true);
                    }
                }
            });
        }
    });

    create_effect({
        let api = api.clone();
        move |_| {
            let api = api.clone();
            spawn_local(async move {
                if let Ok(v) = api.get_app_version().await {
                    set_app_version.set(v);
                }
            });
        }
    });

    let on_reset = Callback::new({
        let api = api.clone();
        move |_: ()| {
            let api = api.clone();
            spawn_local(async move {
                match api.reset_settings().await {
                    Ok(s) => {
                        ui.font_size.set(s.font_size);
                        ui.line_height.set(s.line_height);
                        ui.text_width.set(s.text_width);
                        ui.mirror_mode.set(s.mirror_mode);
                        ui.countdown_seconds.set(s.countdown_seconds);
                        ui.mirror_vertical.set(s.mirror_vertical);
                        ui.reading_guide.set(s.reading_guide_enabled);
                        playback.speed.set(s.scroll_speed);
                        toast.add_success("Settings reset to defaults");
                    }
                    Err(e) => {
                        toast.add_error(&format!("Reset failed: {}", e));
                    }
                }
            });
        }
    });

    let on_save_settings = Callback::new({
        let api = api.clone();
        move |_: ()| {
            let api = api.clone();
            spawn_local(async move {
                let settings = AppSettingsData {
                    font_size: ui.font_size.get(),
                    line_height: ui.line_height.get(),
                    text_width: ui.text_width.get(),
                    scroll_speed: playback.speed.get(),
                    mirror_mode: ui.mirror_mode.get(),
                    theme: ui.theme.get(),
                    countdown_seconds: ui.countdown_seconds.get(),
                    mirror_vertical: ui.mirror_vertical.get(),
                    reading_guide_enabled: ui.reading_guide.get(),
                };
                match api.update_settings(&settings).await {
                    Ok(_) => {
                        toast.add_success("Settings saved");
                    }
                    Err(e) => {
                        toast.add_error(&format!("Save failed: {}", e));
                    }
                }
            });
        }
    });

    view! {
        <div style="padding: 24px; height: 100%; overflow-y: auto;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
                <h1 style="font-size: 24px; color: var(--text-main); margin: 0;">Settings</h1>
                <span style="font-size: 12px; color: var(--text-dim);">
                    "v" {move || app_version.get()}
                </span>
            </div>

            {move || {
                let error = settings_error.get();
                if let Some(ref msg) = error {
                    view! {
                        <div style="
                            background: var(--danger-bg); color: var(--danger-text);
                            padding: 10px 14px; border-radius: 6px;
                            margin-bottom: 12px; font-size: 13px;
                        ">
                            {msg}
                        </div>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {move || if !settings_loaded.get() {
                view! {
                    <div style="color: var(--text-loading); text-align: center; padding: 48px;">
                        "Loading settings..."
                    </div>
                }.into_view()
            } else {
                view! {
                    <div style="display: flex; flex-direction: column; gap: 20px; max-width: 500px;">
                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Font Size: " {move || format!("{:.0}px", ui.font_size.get())}
                            </label>
                            <input
                                type="range"
                                prop:value={move || ui.font_size.get()}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(32.0);
                                    ui.font_size.set(val);
                                }
                                attr:min="12"
                                attr:max="72"
                                attr:step="2"
                                style="width: 100%;"
                            />
                        </div>

                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Line Height: " {move || format!("{:.1}", ui.line_height.get())}
                            </label>
                            <input
                                type="range"
                                prop:value={move || ui.line_height.get()}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(1.8);
                                    ui.line_height.set(val);
                                }
                                attr:min="1.0"
                                attr:max="3.0"
                                attr:step="0.1"
                                style="width: 100%;"
                            />
                        </div>

                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Text Width: " {move || format!("{:.0}ch", ui.text_width.get())}
                            </label>
                            <input
                                type="range"
                                prop:value={move || ui.text_width.get()}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(60.0);
                                    ui.text_width.set(val);
                                }
                                attr:min="20"
                                attr:max="100"
                                attr:step="5"
                                style="width: 100%;"
                            />
                        </div>

                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Scroll Speed: " {move || format!("{:.1}x", playback.speed.get())}
                            </label>
                            <input
                                type="range"
                                prop:value={move || playback.speed.get()}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                    playback.speed.set(val);
                                }
                                attr:min="0.25"
                                attr:max="10"
                                attr:step="0.25"
                                style="width: 100%;"
                            />
                        </div>

                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Countdown (seconds): " {move || format!("{}s", ui.countdown_seconds.get())}
                            </label>
                            <input
                                type="range"
                                prop:value={move || ui.countdown_seconds.get() as f64}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<u32>().unwrap_or(3);
                                    ui.countdown_seconds.set(val);
                                }
                                attr:min="0"
                                attr:max="10"
                                attr:step="1"
                                style="width: 100%;"
                            />
                        </div>

                        <div style="display: flex; align-items: center; gap: 12px;">
                            <label style="font-size: 14px; color: var(--text-muted);">Horizontal Mirror</label>
                            <button
                                on:click=move |_| ui.toggle_mirror()
                                style={move || format!(
                                    "width: 48px; height: 26px; border-radius: 13px; border: none; \
                                     cursor: pointer; transition: background 0.2s; \
                                     background: {}; position: relative;",
                                    if ui.mirror_mode.get() { "var(--accent)" } else { "var(--border-light)" }
                                )}
                            >
                                <div style={move || format!(
                                    "width: 22px; height: 22px; border-radius: 50%; background: #fff; \
                                     position: absolute; top: 2px; transition: left 0.2s; \
                                     left: {};",
                                    if ui.mirror_mode.get() { "24px" } else { "2px" }
                                )}></div>
                            </button>
                        </div>

                        <div style="display: flex; align-items: center; gap: 12px;">
                            <label style="font-size: 14px; color: var(--text-muted);">Vertical Mirror</label>
                            <button
                                on:click=move |_| ui.toggle_mirror_vertical()
                                style={move || format!(
                                    "width: 48px; height: 26px; border-radius: 13px; border: none; \
                                     cursor: pointer; transition: background 0.2s; \
                                     background: {}; position: relative;",
                                    if ui.mirror_vertical.get() { "var(--accent)" } else { "var(--border-light)" }
                                )}
                            >
                                <div style={move || format!(
                                    "width: 22px; height: 22px; border-radius: 50%; background: #fff; \
                                     position: absolute; top: 2px; transition: left 0.2s; \
                                     left: {};",
                                    if ui.mirror_vertical.get() { "24px" } else { "2px" }
                                )}></div>
                            </button>
                        </div>

                        <div style="display: flex; align-items: center; gap: 12px;">
                            <label style="font-size: 14px; color: var(--text-muted);">Reading Guide</label>
                            <button
                                on:click=move |_| ui.toggle_reading_guide()
                                style={move || format!(
                                    "width: 48px; height: 26px; border-radius: 13px; border: none; \
                                     cursor: pointer; transition: background 0.2s; \
                                     background: {}; position: relative;",
                                    if ui.reading_guide.get() { "var(--accent)" } else { "var(--border-light)" }
                                )}
                            >
                                <div style={move || format!(
                                    "width: 22px; height: 22px; border-radius: 50%; background: #fff; \
                                     position: absolute; top: 2px; transition: left 0.2s; \
                                     left: {};",
                                    if ui.reading_guide.get() { "24px" } else { "2px" }
                                )}></div>
                            </button>
                        </div>

                        <div>
                            <label style="display: block; font-size: 14px; color: var(--text-muted); margin-bottom: 6px;">
                                "Theme"
                            </label>
                            <div style="display: flex; gap: 8px;">
                                <button
                                    on:click=move |_| ui.theme.set("Dark".to_string())
                                    style={move || format!(
                                        "padding: 8px 20px; border: 1px solid {}; border-radius: 6px; \
                                         background: {}; color: {}; cursor: pointer; font-size: 14px;",
                                        if ui.theme.get() == "Dark" { "var(--accent)" } else { "var(--button-ghost-border)" },
                                        if ui.theme.get() == "Dark" { "var(--accent)" } else { "transparent" },
                                        if ui.theme.get() == "Dark" { "var(--button-primary-text)" } else { "var(--button-ghost-text)" }
                                    )}
                                >
                                    {"🌙 Dark"}
                                </button>
                                <button
                                    on:click=move |_| ui.theme.set("Light".to_string())
                                    style={move || format!(
                                        "padding: 8px 20px; border: 1px solid {}; border-radius: 6px; \
                                         background: {}; color: {}; cursor: pointer; font-size: 14px;",
                                        if ui.theme.get() == "Light" { "var(--accent)" } else { "var(--button-ghost-border)" },
                                        if ui.theme.get() == "Light" { "var(--accent)" } else { "transparent" },
                                        if ui.theme.get() == "Light" { "var(--button-primary-text)" } else { "var(--button-ghost-text)" }
                                    )}
                                >
                                    {"☀️ Light"}
                                </button>
                            </div>
                        </div>

                        <div style="display: flex; gap: 12px; margin-top: 16px;">
                            <button
                                on:click=move |_| on_save_settings.call(())
                                style="
                                    padding: 10px 24px; border: none; border-radius: 6px;
                                    background: var(--button-primary-bg); color: var(--button-primary-text); cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                Save Settings
                            </button>
                            <button
                                on:click=move |_| on_reset.call(())
                                style="
                                    padding: 10px 24px; border: 1px solid var(--button-ghost-border); border-radius: 6px;
                                    background: transparent; color: var(--button-ghost-text); cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                Reset to Defaults
                            </button>
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
