use crate::bindings::tauri_api::ScriptPlaybackStateData;
use crate::bindings::ApiCtx;
use crate::prompter::engine::start_scroll_loop;
use crate::prompter::keyboard::{handle_keydown, KeyboardActions};
use crate::prompter::mirror::mirror_transform_combined;
use crate::prompter::speed::{
    estimated_reading_seconds, speed_label, speed_presets, validate_speed, word_count, MAX_SPEED,
    MIN_SPEED,
};
use crate::state::app_state::{AppState, View};
use crate::state::playback_state::PlaybackState;
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

fn split_markers(text: &str) -> Vec<(String, bool)> {
    let mut segments = Vec::new();
    let mut remaining = text;
    while let Some(start) = remaining.find('[') {
        if start > 0 {
            segments.push((remaining[..start].to_string(), false));
        }
        let after_bracket = &remaining[start..];
        if let Some(end) = after_bracket.find(']') {
            let marker = &after_bracket[..=end];
            let is_pause = marker.starts_with("[pause") || marker.eq_ignore_ascii_case("[breath]");
            if is_pause {
                segments.push((marker.to_string(), true));
            } else {
                segments.push((marker.to_string(), false));
            }
            remaining = &after_bracket[end + 1..];
        } else {
            segments.push((after_bracket.to_string(), false));
            remaining = "";
        }
    }
    if !remaining.is_empty() {
        segments.push((remaining.to_string(), false));
    }
    segments
}

#[component]
pub fn PrompterView() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    let playback = use_context::<PlaybackState>().expect("PlaybackState not provided");
    let ui = use_context::<UiState>().expect("UiState not provided");
    let toast = expect_context::<ToastState>();
    let api = use_context::<ApiCtx>().expect("AppApi not provided");

    let (settings_loaded, set_settings_loaded) = create_signal(false);
    let (show_controls, set_show_controls) = create_signal(true);
    let (countdown_value, set_countdown_value) = create_signal::<i32>(-1);
    let text_ref = create_node_ref::<leptos::html::Div>();

    let (_custom_speed_text, set_custom_speed_text) = create_signal(String::new());
    let (speed_error, set_speed_error) = create_signal::<Option<String>>(None);
    let (saved_state, set_saved_state) = create_signal::<Option<ScriptPlaybackStateData>>(None);
    let (show_resume_dialog, set_show_resume_dialog) = create_signal(false);

    let hide_timer = Rc::new(Cell::new(None::<i32>));
    let interval_handle = Rc::new(Cell::new(None::<i32>));
    let save_interval = Rc::new(Cell::new(None::<i32>));

    let content = create_resource(move || app_state.selected_script_id.get(), {
        let api = api.clone();
        move |id: Option<String>| {
            let api = api.clone();
            async move {
                match id {
                    None => String::new(),
                    Some(id_val) => api
                        .get_script(&id_val)
                        .await
                        .map(|s| s.content)
                        .unwrap_or_default(),
                }
            }
        }
    });

    create_effect({
        let api = api.clone();
        move |_| {
            let api = api.clone();
            spawn_local(async move {
                if let Ok(settings) = api.get_settings().await {
                    ui.font_size.set(settings.font_size);
                    ui.line_height.set(settings.line_height);
                    ui.text_width.set(settings.text_width);
                    ui.mirror_mode.set(settings.mirror_mode);
                    ui.mirror_vertical.set(settings.mirror_vertical);
                    ui.reading_guide.set(settings.reading_guide_enabled);
                    ui.countdown_seconds.set(settings.countdown_seconds);
                    playback.speed.set(settings.scroll_speed);
                    set_settings_loaded.set(true);
                }
            });
        }
    });

    create_effect({
        let api = api.clone();
        move |_| {
            let id = app_state.selected_script_id.get();
            let loaded = settings_loaded.get();
            if loaded {
                let api = api.clone();
                spawn_local(async move {
                    if let Some(sid) = id {
                        if let Ok(Some(state)) = api.load_playback_state(&sid).await {
                            set_saved_state.set(Some(state));
                            set_show_resume_dialog.set(true);
                        }
                    }
                });
            }
        }
    });

    create_effect(move |_| {
        let alive = start_scroll_loop(
            Signal::from(playback.is_playing),
            playback.scroll_y.write_only(),
            Signal::from(playback.speed),
        );
        on_cleanup(move || {
            alive.set(false);
        });
    });

    create_effect({
        let api = api.clone();
        move |_| {
            let playing = playback.is_playing.get();
            let counting = countdown_value.get() > 0;
            let sid = app_state.selected_script_id.get();
            if playing && !counting {
                if let Some(id_val) = sid {
                    let sc = Rc::clone(&save_interval);
                    let pb2 = playback;
                    let ui2 = ui;
                    let sid2 = id_val.clone();
                    let api = api.clone();
                    let window = web_sys::window().unwrap();
                    let closure: Closure<dyn FnMut()> = Closure::new(move || {
                        let api = api.clone();
                        spawn_local({
                            let pb3 = pb2;
                            let ui3 = ui2;
                            let sid3 = sid2.clone();
                            async move {
                                let _ = api
                                    .save_playback_state(
                                        &sid3,
                                        pb3.scroll_y.get(),
                                        pb3.speed.get(),
                                        Some(ui3.font_size.get()),
                                        None,
                                        Some(ui3.mirror_mode.get()),
                                        Some(ui3.mirror_vertical.get()),
                                    )
                                    .await;
                            }
                        });
                    });
                    let id = window
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            3000,
                        )
                        .unwrap();
                    closure.forget();
                    if let Some(prev) = sc.replace(Some(id)) {
                        let _ = window.clear_interval_with_handle(prev);
                    }
                }
            } else {
                if let Some(h) = save_interval.take() {
                    let window = web_sys::window().unwrap();
                    let _ = window.clear_interval_with_handle(h);
                }
            }
        }
    });

    let prev_playing = Rc::new(Cell::new(false));
    create_effect({
        let api = api.clone();
        move |_| {
            let now_playing = playback.is_playing.get();
            let was_playing = prev_playing.replace(now_playing);
            if was_playing && !now_playing {
                if let Some(sid) = app_state.selected_script_id.get() {
                    let sid2 = sid.clone();
                    let pb2 = playback;
                    let ui2 = ui;
                    let api = api.clone();
                    spawn_local(async move {
                        let _ = api
                            .save_playback_state(
                                &sid2,
                                pb2.scroll_y.get(),
                                pb2.speed.get(),
                                Some(ui2.font_size.get()),
                                None,
                                Some(ui2.mirror_mode.get()),
                                Some(ui2.mirror_vertical.get()),
                            )
                            .await;
                    });
                }
            }
        }
    });

    create_effect(move |_| {
        let playing = playback.is_playing.get();
        let counting = countdown_value.get() > 0;
        if playing && !counting {
            let pb = playback.clone();
            let set_ctrl = set_show_controls;
            let htimer = Rc::clone(&hide_timer);
            let handle = set_timeout(
                move || {
                    if pb.is_playing.get() {
                        set_ctrl.set(false);
                    }
                },
                std::time::Duration::from_secs(3),
            );
            if let Some(prev) = htimer.replace(Some(handle)) {
                clear_timeout(prev);
            }
            on_cleanup(move || {
                if let Some(h) = htimer.take() {
                    clear_timeout(h);
                }
            });
        } else {
            set_show_controls.set(true);
        }
    });

    let playback_c = playback.clone();
    let app_state_c = app_state.clone();
    let ui_c = ui.clone();
    let set_countdown_value_c = set_countdown_value;

    create_effect(move |_| {
        let window = web_sys::window().unwrap();
        let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
            let actions = KeyboardActions {
                toggle_play: Box::new({
                    let scv = set_countdown_value_c;
                    let pb = playback_c;
                    move || {
                        let cd = countdown_value.get();
                        if cd > 0 {
                            scv.set(-1);
                            pb.scroll_y.set(0.0);
                            return;
                        }
                        pb.toggle_play();
                    }
                }),
                exit_prompter: Box::new({
                    let av = app_state_c;
                    let pb = playback_c;
                    let scv = set_countdown_value_c;
                    move || {
                        exit_fullscreen_if_open();
                        scv.set(-1);
                        pb.is_playing.set(false);
                        pb.scroll_y.set(0.0);
                        av.view.set(View::Library);
                    }
                }),
                toggle_fullscreen: Box::new(toggle_fullscreen),
                restart: Box::new({
                    let pb = playback_c;
                    move || {
                        pb.scroll_y.set(0.0);
                        pb.is_playing.set(true);
                    }
                }),
                increase_speed: Box::new({
                    let pb = playback_c;
                    move || pb.increase_speed()
                }),
                decrease_speed: Box::new({
                    let pb = playback_c;
                    move || pb.decrease_speed()
                }),
                jump_forward: Box::new({
                    let pb = playback_c;
                    move || {
                        pb.jump_forward();
                        toast.add_info("+5s");
                    }
                }),
                jump_backward: Box::new({
                    let pb = playback_c;
                    move || {
                        pb.jump_backward();
                        toast.add_info("-5s");
                    }
                }),
                jump_big_forward: Box::new({
                    let pb = playback_c;
                    move || {
                        pb.jump_big_forward();
                        toast.add_info("+20s");
                    }
                }),
                jump_big_backward: Box::new({
                    let pb = playback_c;
                    move || {
                        pb.jump_big_backward();
                        toast.add_info("-20s");
                    }
                }),
                toggle_mirror: Box::new({
                    let u = ui_c;
                    move || u.toggle_mirror()
                }),
                toggle_mirror_vertical: Box::new({
                    let u = ui_c;
                    move || u.toggle_mirror_vertical()
                }),
                increase_font_size: Box::new({
                    let u = ui_c;
                    move || u.increase_font_size()
                }),
                decrease_font_size: Box::new({
                    let u = ui_c;
                    move || u.decrease_font_size()
                }),
                toggle_shortcut_help: Box::new({
                    let u = ui_c;
                    move || u.toggle_shortcut_help()
                }),
            };
            handle_keydown(&event, &actions);
        });
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        on_cleanup(move || {
            let _ = window
                .remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        });
    });

    let interval_handle_c = Rc::clone(&interval_handle);
    let ih_controls = Rc::clone(&interval_handle);

    let progress_pct = move || {
        let total = text_ref
            .get()
            .map(|e| e.scroll_height() as f64)
            .unwrap_or(1.0);
        let current = playback.scroll_y.get().max(0.0).min(total);
        ((current / total.max(1.0)) * 100.0).min(100.0)
    };

    let estimated_remaining = move || {
        let total = text_ref
            .get()
            .map(|e| e.scroll_height() as f64)
            .unwrap_or(1.0);
        let remaining = (total - playback.scroll_y.get()).max(0.0);
        let spd = playback.speed.get().max(0.25);
        let secs = (remaining / (spd * 60.0)).round() as u32;
        let m = secs / 60;
        let s = secs % 60;
        if m > 0 {
            format!("{}m {:02}s", m, s)
        } else {
            format!("{}s", s)
        }
    };

    let container_style = move || {
        format!(
            "width: 100vw; height: 100vh; \
             background: #000; color: #fff; \
             display: flex; flex-direction: column; \
             overflow: hidden; position: relative; \
             transform: {}; \
             font-family: 'Segoe UI', system-ui, sans-serif;",
            mirror_transform_combined(
                Signal::from(ui.mirror_mode),
                Signal::from(ui.mirror_vertical)
            ),
        )
    };

    let text_style = move || {
        format!(
            "font-size: {}px; line-height: {}; \
             max-width: {}ch; margin: 0 auto; \
             padding: 0 24px; white-space: pre-wrap; \
             transform: translate3d(0, -{}px, 0); \
             will-change: transform; \
             transition: none;",
            ui.font_size.get(),
            ui.line_height.get(),
            ui.text_width.get(),
            playback.scroll_y.get(),
        )
    };

    view! {
        <div style={container_style} on:mousemove=move |_| set_show_controls.set(true)>

            {move || {
                if ui.reading_guide.get() {
                    view! {
                        <div style="
                            position: fixed; top: 0; left: 0; right: 0; bottom: 0;
                            pointer-events: none; z-index: 5;
                            display: flex; align-items: center; justify-content: center;
                        ">
                            <div style="
                                position: absolute; left: 5%; right: 5%;
                                height: 60px; background: rgba(255,255,255,0.04);
                                border-top: 1px solid rgba(255,255,255,0.1);
                                border-bottom: 1px solid rgba(255,255,255,0.1);
                                border-radius: 4px;
                            "></div>
                        </div>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {move || {
                let cv = countdown_value.get();
                if cv > 0 {
                    let show_get_ready = cv > 3;
                    view! {
                        <div style="
                            position: fixed; inset: 0; z-index: 100;
                            display: flex; flex-direction: column; align-items: center; justify-content: center;
                            background: #000;
                        ">
                            {if show_get_ready {
                                Some(view! {
                                    <span style="font-size: 20px; color: #888; margin-bottom: 16px; user-select: none;">
                                        "Get ready"
                                    </span>
                                }.into_view())
                            } else { None }}
                            <span style="font-size: 120px; font-weight: bold; color: #fff; user-select: none;">
                                {cv}
                            </span>
                        </div>
                    }.into_view()
                } else if cv == 0 {
                    view! {
                        <div style="
                            position: fixed; inset: 0; z-index: 100;
                            display: flex; align-items: center; justify-content: center;
                            background: #000;
                        ">
                            <span style="font-size: 48px; font-weight: bold; color: #e94560; user-select: none;">
                                "GO"
                            </span>
                        </div>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {
                let api = api.clone();
                move || {
                if show_resume_dialog.get() && countdown_value.get() < 0 {
                    let st = saved_state.get();
                    if let Some(s) = st {
                        let scroll_pct = ((s.scroll_offset_px / text_ref.get().map(|e| e.scroll_height() as f64).unwrap_or(1.0).max(1.0)) * 100.0).min(100.0);
                        view! {
                            <div style="
                                position: fixed; inset: 0; z-index: 110;
                                display: flex; align-items: center; justify-content: center;
                                background: rgba(0,0,0,0.85);
                            ">
                                <div style="
                                    background: #1a1a2e; padding: 24px 32px;
                                    border-radius: 12px; border: 1px solid #333;
                                    text-align: center; max-width: 360px;
                                ">
                                    <div style="font-size: 14px; color: #e0e0e0; margin-bottom: 12px;">
                                        "Resume where you left off?"
                                    </div>
                                    <div style="font-size: 12px; color: #888; margin-bottom: 20px;">
                                        {format!("You were at ~{:.0}% of the script.", scroll_pct)}
                                    </div>
                                    <div style="display: flex; gap: 10px; justify-content: center;">
                                        <button
                                            on:click=move |_| {
                                                let sv = s.clone();
                                                set_show_resume_dialog.set(false);
                                                playback.scroll_y.set(sv.scroll_offset_px);
                                                playback.speed.set(sv.speed_multiplier);
                                                if let Some(fs) = sv.font_size { ui.font_size.set(fs); }
                                                if let Some(mm) = sv.mirror_mode { ui.mirror_mode.set(mm); }
                                                if let Some(mv) = sv.mirror_vertical { ui.mirror_vertical.set(mv); }
                                            }
                                            style="
                                                padding: 10px 22px; border: none; border-radius: 6px;
                                                background: #e94560; color: white; cursor: pointer; font-size: 14px;
                                            "
                                        >
                                            "▶ Resume"
                                        </button>
                                        <button
                                            on:click={
                                                let api = api.clone();
                                                move |_| {
                                                set_show_resume_dialog.set(false);
                                                if let Some(sid) = app_state.selected_script_id.get() {
                                                    let api = api.clone();
                                                    spawn_local(async move {
                                                        let _ = api.clear_playback_state(&sid).await;
                                                    });
                                                }
                                                }
                                            }
                                            style="
                                                padding: 10px 22px; border: 1px solid #555; border-radius: 6px;
                                                background: transparent; color: #ccc; cursor: pointer; font-size: 14px;
                                            "
                                        >
                                            "Start from beginning"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {
                let api = api.clone();
                move || {
                let show = show_controls.get() || !playback.is_playing.get() || countdown_value.get() > 0;
                if show {
                    view! {
                            <div style="
                                position: fixed; top: 0; left: 0; right: 0;
                                padding: 8px 16px;
                                display: flex; justify-content: space-between;
                                align-items: center; z-index: 50;
                                background: linear-gradient(180deg, rgba(0,0,0,0.85) 0%, transparent 100%);
                            ">
                                <div style="display: flex; gap: 6px; align-items: center;">
                                    <button on:click={
                                        let cd = countdown_value;
                                        let scd = set_countdown_value;
                                        let pb = playback;
                                        let sl = settings_loaded;
                                        let ui2 = ui;
                                        let ih = Rc::clone(&ih_controls);
                                        move |_: leptos::ev::MouseEvent| {
                                            let c = cd.get();
                                            if c > 0 {
                                                clear_countdown_interval(&ih);
                                                scd.set(-1);
                                                pb.scroll_y.set(0.0);
                                                return;
                                            }
                                            if pb.is_playing.get() {
                                                pb.toggle_play();
                                                return;
                                            }
                                            if sl.get() && pb.scroll_y.get() == 0.0 {
                                                let secs = ui2.countdown_seconds.get() as i32;
                                                if secs > 0 {
                                                    start_countdown(secs, pb, scd, cd, &ih);
                                                    return;
                                                }
                                            }
                                            pb.toggle_play();
                                        }
                                    }
                                        style="padding: 6px 14px; border: 1px solid #666; border-radius: 4px;
                                               background: transparent; color: #fff; cursor: pointer; font-size: 13px;
                                               min-width: 60px;">
                                        {move || {
                                            if countdown_value.get() > 0 { "Cancel" }
                                            else if playback.is_playing.get() { "⏸ Pause" }
                                            else { "▶ Play" }
                                        }}
                                    </button>
                                    <button on:click={
                                let pb = playback;
                                let scd = set_countdown_value;
                                let cd = countdown_value;
                                let sl = settings_loaded;
                                let ui2 = ui;
                                 let ih = Rc::clone(&ih_controls);
                                 move |_: leptos::ev::MouseEvent| {
                                     clear_countdown_interval(&ih);
                                     scd.set(-1);
                                     pb.scroll_y.set(0.0);
                                     pb.is_playing.set(false);
                                     if sl.get() {
                                         let secs = ui2.countdown_seconds.get() as i32;
                                         if secs > 0 {
                                             start_countdown(secs, pb, scd, cd, &ih);
                                             return;
                                         }
                                     }
                                     pb.is_playing.set(true);
                                 }
                                    }
                                        style="padding: 4px 10px; border: 1px solid #555; border-radius: 4px;
                                               background: transparent; color: #ccc; cursor: pointer; font-size: 11px;">
                                        "↺ Restart"
                                    </button>
                                    <button on:click=move |_| set_show_controls.set(false)
                                        style="padding: 4px 8px; border: none; border-radius: 4px;
                                               background: transparent; color: #555; cursor: pointer; font-size: 10px;">
                                        "Hide"
                                    </button>
                                    <button on:click={
                                        let ap = app_state;
                                        let pb = playback;
                                        let api = api.clone();
                                        move |_: leptos::ev::MouseEvent| {
                                            pb.scroll_y.set(0.0);
                                            if let Some(sid) = ap.selected_script_id.get() {
                                                let api = api.clone();
                                                spawn_local(async move {
                                                    let _ = api.clear_playback_state(&sid).await;
                                                });
                                            }
                                        }
                                    }
                                        style="padding: 4px 10px; border: 1px solid #555; border-radius: 4px;
                                               background: transparent; color: #888; cursor: pointer; font-size: 10px;">
                                        "Reset"
                                    </button>
                                </div>
                                <div style="display: flex; gap: 12px; align-items: center;">
                                    <span style="font-size: 11px; color: #888;">
                                        {move || format!("{:.0}%", progress_pct())}
                                    </span>
                                    <span style="font-size: 11px; color: #666;">
                                        {move || estimated_remaining()}
                                    </span>
                                    <span style="font-size: 11px; color: #e94560; min-width: 48px; text-align: right;">
                                        {move || speed_label(playback.speed.get())}
                                    </span>
                                    <span style="font-size: 11px; color: #666; min-width: 32px; text-align: right;">
                                        {move || format!("{:.1}x", playback.speed.get())}
                                    </span>
                                    <button on:click=move |_| toggle_fullscreen()
                                        style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                               background: transparent; color: #ccc; cursor: pointer; font-size: 11px;">
                                        "⛶"
                                    </button>
                                    <button on:click={
                                        let ap = app_state;
                                        let pb = playback;
                                        let scd = set_countdown_value;
                                        let ui2 = ui;
                                         let ih = Rc::clone(&ih_controls);
                                         let api = api.clone();
                                         move |_: leptos::ev::MouseEvent| {
                                             exit_fullscreen_if_open();
                                             clear_countdown_interval(&ih);
                                             scd.set(-1);
                                             if let Some(sid) = ap.selected_script_id.get() {
                                                 let sid2 = sid.clone();
                                                 let pb2 = pb;
                                                 let ui3 = ui2;
                                                 let api = api.clone();
                                                 spawn_local(async move {
                                                     let _ = api.save_playback_state(
                                                         &sid2,
                                                         pb2.scroll_y.get(),
                                                         pb2.speed.get(),
                                                         Some(ui3.font_size.get()),
                                                         None,
                                                         Some(ui3.mirror_mode.get()),
                                                         Some(ui3.mirror_vertical.get()),
                                                     )
                                                     .await;
                                                 });
                                             }
                                             pb.is_playing.set(false);
                                             pb.scroll_y.set(0.0);
                                             ap.view.set(View::Library);
                                         }
                                    }
                                        style="padding: 6px 12px; border: none; border-radius: 4px;
                                               background: #e94560; color: white; cursor: pointer; font-size: 12px;">
                                        "✕ Exit"
                                    </button>
                                </div>
                            </div>

                        <div style="
                            position: fixed; bottom: 0; left: 0; right: 0;
                            display: flex; justify-content: center; gap: 4px;
                            padding: 8px 16px; z-index: 50;
                            background: linear-gradient(0deg, rgba(0,0,0,0.85) 0%, transparent 100%);
                        ">
                            {move || {
                                if ui.rehearse_mode.get() && !playback.is_playing.get() {
                                    view! {
                                        <span style="padding: 4px 8px; color: #e94560; font-size: 10px; font-weight: bold;">"🎤 REHEARSAL"</span>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }
                            }}
                            <button on:click=move |_| ui.toggle_mirror()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                {move || if ui.mirror_mode.get() { "Mir:H" } else { "Mir" }}
                            </button>
                            <button on:click=move |_| ui.toggle_mirror_vertical()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                {move || if ui.mirror_vertical.get() { "Flip:V" } else { "Flip" }}
                            </button>
                            <button on:click=move |_| ui.toggle_reading_guide()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                {move || if ui.reading_guide.get() { "Gd:ON" } else { "Guide" }}
                            </button>
                            <span style="width: 1px; height: 16px; background: #444; margin: 0 4px;"></span>
                            {speed_presets().into_iter().map(|(label, val)| {
                                let pb = playback;
                                let v = val;
                                view! {
                                    <button
                                        on:click=move |_| pb.set_speed(v)
                                        style={move || {
                                            let current = pb.speed.get();
                                            let active = (current - v).abs() < 0.01;
                                            format!(
                                                "padding: 4px 8px; border: 1px solid {}; border-radius: 4px; \
                                                 background: {}; color: {}; cursor: pointer; font-size: 10px;",
                                                if active { "#e94560" } else { "#555" },
                                                if active { "rgba(233,69,96,0.25)" } else { "transparent" },
                                                if active { "#e94560" } else { "#aaa" },
                                            )
                                        }}
                                    >
                                        {label}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                            <span style="width: 1px; height: 16px; background: #444; margin: 0 4px;"></span>
                            <input
                                type="text"
                                placeholder="speed"
                                prop:value={move || format!("{:.2}", playback.speed.get())}
                                on:input=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_custom_speed_text.set(val.clone());
                                    match val.parse::<f64>().ok().and_then(validate_speed) {
                                        Some(v) => {
                                            set_speed_error.set(None);
                                            playback.speed.set(v);
                                        }
                                        None => {
                                            if val.is_empty() {
                                                set_speed_error.set(None);
                                            } else {
                                                set_speed_error.set(Some(format!(
                                                    "Enter {}-{}", MIN_SPEED, MAX_SPEED
                                                )));
                                            }
                                        }
                                    }
                                }
                                style="
                                    width: 52px; padding: 2px 6px; border: 1px solid #555; border-radius: 4px;
                                    background: #111; color: #fff; font-size: 11px; text-align: center;
                                    outline: none;
                                "
                            />
                            {move || {
                                let err = speed_error.get();
                                if let Some(msg) = err {
                                    view! {
                                        <span style="
                                            position: fixed; bottom: 48px; left: 50%; transform: translateX(-50%);
                                            padding: 4px 12px; border-radius: 4px;
                                            background: rgba(233,69,96,0.9); color: #fff;
                                            font-size: 10px; z-index: 70; white-space: nowrap;
                                        ">{msg}</span>
                                    }.into_view()
                                } else {
                                    view! { <span></span> }.into_view()
                                }
                            }}
                            <button on:click=move |_| playback.decrease_speed()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                "◀◀"
                            </button>
                            <button on:click=move |_| playback.increase_speed()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                "▶▶"
                            </button>
                            <span style="width: 1px; height: 16px; background: #444; margin: 0 4px;"></span>
                            <button on:click=move |_| ui.decrease_font_size()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                "A-"
                            </button>
                            <button on:click=move |_| ui.increase_font_size()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                "A+"
                            </button>
                            <button on:click=move |_| ui.toggle_shortcut_help()
                                style="padding: 4px 8px; border: 1px solid #555; border-radius: 4px;
                                       background: transparent; color: #aaa; cursor: pointer; font-size: 11px;">
                                "?"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {move || {
                if ui.show_shortcut_help.get() {
                    view! {
                        <div style="
                            position: fixed; inset: 0; z-index: 200;
                            display: flex; align-items: center; justify-content: center;
                            background: rgba(0,0,0,0.85);
                        " on:click=move |_| ui.toggle_shortcut_help()>
                            <div style="
                                background: #1a1a2e; padding: 24px 32px;
                                border-radius: 12px; border: 1px solid #333;
                                max-width: 400px;
                            " on:click=move |ev| ev.stop_propagation()>
                                <h2 style="color: #e0e0e0; margin: 0 0 16px 0; font-size: 18px;">
                                    "Keyboard Shortcuts"
                                </h2>
                                <table style="width: 100%; border-collapse: collapse; font-size: 13px;">
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">Space</td>
                                        <td style="color: #aaa;">Play / Pause</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">Esc</td>
                                        <td style="color: #aaa;">Exit prompter</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">F</td>
                                        <td style="color: #aaa;">Toggle fullscreen</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">R</td>
                                        <td style="color: #aaa;">Restart from top</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">Up / Down</td>
                                        <td style="color: #aaa;">Speed up / down</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">Left / Right</td>
                                        <td style="color: #aaa;">Jump back / forward (5s)</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">Shift+Left / Right</td>
                                        <td style="color: #aaa;">Big jump (20s)</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">M</td>
                                        <td style="color: #aaa;">Mirror horizontal</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">V</td>
                                        <td style="color: #aaa;">Mirror vertical</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">+ / -</td>
                                        <td style="color: #aaa;">Increase / decrease font</td></tr>
                                    <tr><td style="color: #e94560; padding: 4px 12px 4px 0;">H</td>
                                        <td style="color: #aaa;">Toggle shortcuts help</td></tr>
                                </table>
                                <p style="color: #555; font-size: 11px; margin: 12px 0 0 0;">
                                    "Click outside or press H to close."
                                </p>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}

            {{
                let ih_rehearse = Rc::clone(&interval_handle_c);
                move || {
                    if ui.rehearse_mode.get() && !playback.is_playing.get() && countdown_value.get() < 0 {
                        let text = content.get().unwrap_or_default();
                        let wc = word_count(&text);
                        let est_secs = estimated_reading_seconds(&text, 130.0);
                        let est_m = est_secs / 60;
                        let est_s = est_secs % 60;
                        let time_str = if est_m > 0 {
                            format!("{}m {:02}s", est_m, est_s)
                        } else {
                            format!("{}s", est_s)
                        };
                        let ih_inner = Rc::clone(&ih_rehearse);
                        view! {
                            <div style="
                                position: fixed; inset: 0; z-index: 90;
                                display: flex; align-items: center; justify-content: center;
                                background: #000;
                            ">
                                <div style="text-align: center;">
                                    <div style="font-size: 14px; color: #e94560; margin-bottom: 8px;">"🎤 REHEARSAL MODE"</div>
                                    <div style="font-size: 28px; font-weight: bold; color: #fff; margin-bottom: 16px;">
                                        {wc}
                                        <span style="font-size: 16px; color: #888; font-weight: normal;">" words"</span>
                                    </div>
                                    <div style="font-size: 16px; color: #aaa; margin-bottom: 24px;">
                                        "Estimated reading time: " {time_str}
                                        <span style="font-size: 12px; color: #666;">" (at ~130 WPM)"</span>
                                    </div>
                                    <button
                                        on:click=move |_| {
                                            let secs = ui.countdown_seconds.get() as i32;
                                            if secs > 0 {
                                                start_countdown(secs, playback, set_countdown_value, countdown_value, &ih_inner);
                                            } else {
                                                playback.is_playing.set(true);
                                            }
                                        }
                                        style="
                                            padding: 12px 32px; border: none; border-radius: 8px;
                                            background: #e94560; color: #fff; cursor: pointer; font-size: 18px;
                                        "
                                    >
                                        "▶ Start Rehearsal"
                                    </button>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }
                }
            }}

            <div style="flex: 1; overflow: hidden; padding-top: 60px; padding-bottom: 20px;">
                <div id="prompter-text" node_ref=text_ref style={text_style}>
                    {move || {
                        let text = content.get().unwrap_or_default();
                        let segments = split_markers(&text);
                        segments.into_iter().map(|(seg, is_marker)| {
                            if is_marker {
                                let display: String = if seg.eq_ignore_ascii_case("[breath]") {
                                    " 🌬️ ".to_string()
                                } else if seg.starts_with("[pause") {
                                    let dur = seg.trim_start_matches("[pause")
                                        .trim_start_matches(':')
                                        .trim_end_matches(']');
                                    if dur.is_empty() { " ⏸ ".to_string() } else { format!(" [{}s] ", dur) }
                                } else {
                                    seg.to_string()
                                };
                                view! {
                                    <span style="
                                        background: rgba(233,69,96,0.2);
                                        color: #e94560; font-weight: 500;
                                        border-radius: 4px; padding: 0 4px;
                                    ">{display}</span>
                                }.into_view()
                            } else {
                                view! { <span>{seg}</span> }.into_view()
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

        </div>
    }
}

fn start_countdown(
    seconds: i32,
    playback: PlaybackState,
    set_countdown_value: WriteSignal<i32>,
    countdown_value: ReadSignal<i32>,
    interval_handle: &Rc<Cell<Option<i32>>>,
) {
    clear_countdown_interval(interval_handle);
    set_countdown_value.set(seconds);
    playback.scroll_y.set(0.0);
    playback.is_playing.set(false);

    let scv = set_countdown_value;
    let cv = countdown_value;
    let pb = playback;
    let window = web_sys::window().unwrap();
    let win = window.clone();
    let handle = Rc::new(Cell::new(None::<i32>));
    let h = Rc::clone(&handle);

    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        let current = cv.get();
        if current < 0 {
            if let Some(id) = h.get() {
                let _ = win.clear_interval_with_handle(id);
            }
            return;
        }
        if current <= 1 {
            scv.set(-1);
            pb.is_playing.set(true);
            if let Some(id) = h.get() {
                let _ = win.clear_interval_with_handle(id);
            }
        } else {
            scv.set(current - 1);
        }
    });

    let id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();

    handle.set(Some(id));
    interval_handle.set(Some(id));
    closure.forget();
}

fn clear_countdown_interval(handle: &Rc<Cell<Option<i32>>>) {
    if let Some(id) = handle.take() {
        let window = web_sys::window().unwrap();
        let _ = window.clear_interval_with_handle(id);
    }
}

fn toggle_fullscreen() {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if doc.fullscreen_element().is_some() {
        let _ = doc.exit_fullscreen();
    } else if let Some(elem) = doc.document_element() {
        let _ = elem.request_fullscreen();
    }
}

fn exit_fullscreen_if_open() {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if doc.fullscreen_element().is_some() {
        let _ = doc.exit_fullscreen();
    }
}

use wasm_bindgen_futures::spawn_local;

fn set_timeout<F: FnOnce() + 'static>(f: F, dur: std::time::Duration) -> i32 {
    let window = web_sys::window().unwrap();
    let closure = Closure::once(f);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            dur.as_millis() as i32,
        )
        .unwrap()
}

fn clear_timeout(handle: i32) {
    let window = web_sys::window().unwrap();
    let _ = window.clear_timeout_with_handle(handle);
}
