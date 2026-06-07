use crate::bindings::ApiCtx;
use crate::state::app_state::{AppState, View};
use crate::state::toast::ToastState;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    fn set_timeout(handler: &JsValue, timeout: i32) -> i32;
    #[wasm_bindgen(js_name = "clearTimeout")]
    fn clear_timeout(handle: i32);
}

#[derive(Clone)]
enum SaveStatus {
    Idle,
    Unsaved,
    Saving,
    Saved,
    Error(String),
}

#[component]
pub fn ScriptEditor() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    let toast = expect_context::<ToastState>();
    let api = use_context::<ApiCtx>().expect("AppApi not provided");

    let editing_id = move || app_state.editing_script_id.get();

    let script_data = create_resource(editing_id, {
        let api = api.clone();
        move |id| {
            let api = api.clone();
            async move {
                match id {
                    None => Ok(None),
                    Some(id) => api.get_script(&id).await.map(Some),
                }
            }
        }
    });

    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (save_status, set_save_status) = create_signal(SaveStatus::Idle);
    let debounce_handle = create_rw_signal::<Option<i32>>(None);
    let has_initialized = create_rw_signal(false);
    let is_existing = create_rw_signal(false);

    let schedule_auto_save = Callback::new({
        let api = api.clone();
        move |_: ()| {
            if let Some(handle) = debounce_handle.get() {
                clear_timeout(handle);
            }
            set_save_status.set(SaveStatus::Unsaved);
            let api = api.clone();
            let cb = Closure::wrap(Box::new(move || {
                set_save_status.set(SaveStatus::Saving);
                let id = app_state.editing_script_id.get();
                let t = title.get();
                let c = content.get();
                if t.trim().is_empty() {
                    set_save_status.set(SaveStatus::Idle);
                    return;
                }
                let api = api.clone();
                spawn_local(async move {
                    let result = match id {
                        None => api.create_script(&t, &c).await,
                        Some(ref id) => api.update_script(id, &t, &c).await,
                    };
                    match result {
                        Ok(script) => {
                            app_state.selected_script_id.set(Some(script.id.clone()));
                            app_state.editing_script_id.set(Some(script.id));
                            app_state.refresh_library();
                            set_save_status.set(SaveStatus::Saved);
                            toast.add_success("Saved");
                        }
                        Err(e) => {
                            set_save_status.set(SaveStatus::Error(e.clone()));
                            toast.add_error(&format!("Save failed: {}", e));
                        }
                    }
                });
            }) as Box<dyn Fn()>);
            let handle = set_timeout(cb.as_ref().unchecked_ref(), 500);
            cb.forget();
            debounce_handle.set(Some(handle));
        }
    });

    let on_title_input = move |ev: leptos::ev::Event| {
        let v = event_target_value(&ev);
        set_title.set(v);
        if is_existing.get() {
            schedule_auto_save.call(());
        }
    };

    let on_content_input = move |ev: leptos::ev::Event| {
        let v = event_target_value(&ev);
        set_content.set(v);
        if is_existing.get() {
            schedule_auto_save.call(());
        }
    };

    create_effect(move |_| {
        script_data.get();
    });

    let perform_save = Callback::new({
        let api = api.clone();
        move |_: ()| {
            let t = title.get();
            let c = content.get();
            if t.trim().is_empty() {
                set_save_status.set(SaveStatus::Error("Title cannot be empty.".to_string()));
                return;
            }
            set_save_status.set(SaveStatus::Saving);
            let api = api.clone();
            spawn_local(async move {
                let id = app_state.editing_script_id.get();
                let result = match id {
                    None => api.create_script(&t, &c).await,
                    Some(ref id) => api.update_script(id, &t, &c).await,
                };
                match result {
                    Ok(script) => {
                        app_state.selected_script_id.set(Some(script.id.clone()));
                        app_state.editing_script_id.set(Some(script.id));
                        app_state.refresh_library();
                        is_existing.set(true);
                        set_save_status.set(SaveStatus::Saved);
                    }
                    Err(e) => {
                        set_save_status.set(SaveStatus::Error(e));
                    }
                }
            });
        }
    });

    let on_save = move |_: leptos::ev::MouseEvent| {
        perform_save.call(());
    };

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "s" && (ev.ctrl_key() || ev.meta_key()) {
            ev.prevent_default();
            perform_save.call(());
        }
    };

    let on_back = move |_: leptos::ev::MouseEvent| {
        app_state.view.set(View::Library);
    };

    let on_start_prompter = {
        let api = api.clone();
        move |_: leptos::ev::MouseEvent| {
            let t = title.get();
            let c = content.get();
            if !t.trim().is_empty() {
                if is_existing.get() {
                    if let Some(handle) = debounce_handle.get() {
                        clear_timeout(handle);
                        debounce_handle.set(None);
                    }
                    set_save_status.set(SaveStatus::Saving);
                    let api = api.clone();
                    spawn_local(async move {
                        let id = app_state.editing_script_id.get();
                        let result = match id {
                            None => api.create_script(&t, &c).await,
                            Some(ref id) => api.update_script(id, &t, &c).await,
                        };
                        if let Ok(script) = result {
                            app_state.selected_script_id.set(Some(script.id.clone()));
                            app_state.editing_script_id.set(Some(script.id));
                            is_existing.set(true);
                        }
                        app_state.view.set(View::Prompter);
                    });
                } else {
                    set_save_status.set(SaveStatus::Saving);
                    let api = api.clone();
                    spawn_local(async move {
                        if let Ok(script) = api.create_script(&t, &c).await {
                            app_state.selected_script_id.set(Some(script.id.clone()));
                            app_state.editing_script_id.set(Some(script.id));
                            is_existing.set(true);
                        }
                        app_state.view.set(View::Prompter);
                    });
                }
            } else {
                app_state.view.set(View::Prompter);
            }
        }
    };

    let save_status_text = move || match save_status.get() {
        SaveStatus::Idle => String::new(),
        SaveStatus::Unsaved => "Unsaved changes".to_string(),
        SaveStatus::Saving => "Saving...".to_string(),
        SaveStatus::Saved => "Saved".to_string(),
        SaveStatus::Error(ref e) => format!("Error: {}", e),
    };

    view! {
        <div style="padding: 24px; height: 100%; display: flex; flex-direction: column;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                <button
                    on:click=on_back
                    style="
                        padding: 8px 16px; border: 1px solid var(--button-ghost-border); border-radius: 6px;
                        background: transparent; color: var(--button-ghost-text); cursor: pointer;
                        font-size: 14px;
                    "
                >
                    {"← Back"}
                </button>
                <div style="display: flex; gap: 8px; align-items: center;">
                    {move || {
                        let text = save_status_text();
                        if text.is_empty() {
                            return view! { <span></span> }.into_view();
                        }
                        let color = match save_status.get() {
                            SaveStatus::Idle => "var(--text-loading)",
                            SaveStatus::Unsaved => "var(--warning-text)",
                            SaveStatus::Saving => "var(--text-loading)",
                            SaveStatus::Saved => "var(--success-text)",
                            SaveStatus::Error(_) => "var(--danger-text)",
                        };
                        view! {
                            <span style=format!("color: {}; font-size: 13px;", color)>
                                {text}
                            </span>
                        }.into_view()
                    }}
                    <span style="color: var(--text-dim); font-size: 11px; user-select: none;">
                        "Ctrl+S"
                    </span>
                    <button
                        on:click=on_save
                        style="
                            padding: 8px 16px; border: none; border-radius: 6px;
                            background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer;
                            font-size: 14px;
                        "
                    >
                        Save
                    </button>
                    <button
                        on:click=on_start_prompter
                        style="
                            padding: 8px 16px; border: none; border-radius: 6px;
                            background: var(--button-primary-bg); color: var(--button-primary-text); cursor: pointer;
                            font-size: 14px;
                        "
                    >
                        Open in Prompter
                    </button>
                </div>
            </div>

            {move || {
                match save_status.get() {
                    SaveStatus::Error(ref msg) => {
                        view! {
                            <div style="
                                background: var(--danger-bg); color: var(--danger-text);
                                padding: 10px 14px; border-radius: 6px;
                                margin-bottom: 12px; font-size: 13px;
                            ">
                                {msg}
                            </div>
                        }.into_view()
                    }
                    _ => view! { <span></span> }.into_view(),
                }
            }}

            {move || match script_data.get() {
                None => view! {
                    <div style="color: var(--text-loading); text-align: center; padding: 48px;">
                        "Loading..."
                    </div>
                }.into_view(),
                Some(Err(e)) => view! {
                    <div style="color: var(--danger-text); padding: 16px; text-align: center;">
                        "Error: " {e}
                    </div>
                }.into_view(),
                Some(Ok(data)) => {
                    let (initial_title, initial_content) = match data {
                        None => {
                            is_existing.set(false);
                            (String::new(), String::new())
                        }
                        Some(s) => {
                            is_existing.set(true);
                            (s.title.clone(), s.content.clone())
                        }
                    };

                    if !has_initialized.get() {
                        has_initialized.set(true);
                        set_title.set(initial_title.clone());
                        set_content.set(initial_content.clone());
                    }

                    view! {
                        <div style="display: flex; flex-direction: column; gap: 12px; flex: 1;">
                            <input
                                prop:value=move || title.get()
                                on:input=on_title_input
                                on:keydown=on_keydown
                                placeholder="Script title"
                                style="
                                    width: 100%; padding: 10px 14px;
                                    border: 1px solid var(--input-border); border-radius: 8px;
                                    background: var(--input-bg); color: var(--input-text);
                                    font-size: 18px; font-weight: 600;
                                    outline: none;
                                "
                            />
                            <textarea
                                prop:value=move || content.get()
                                on:input=on_content_input
                                on:keydown=on_keydown
                                placeholder="Script content..."
                                style="
                                    flex: 1; width: 100%; padding: 14px;
                                    border: 1px solid var(--input-border); border-radius: 8px;
                                    background: var(--input-bg); color: var(--input-text);
                                    font-size: 16px; line-height: 1.6;
                                    resize: none; outline: none;
                                    font-family: inherit;
                                "
                            ></textarea>
                            <div style="display: flex; gap: 16px; flex-wrap: wrap; padding: 6px 0; font-size: 11px; color: var(--text-muted2); user-select: none;">
                                <span>"💡 Use [pause] or [pause:3] to mark pauses"</span>
                                <span>"💡 Use [breath] for a breath cue"</span>
                                <span>"💡 Ctrl+S to save"</span>
                                <span>"💡 Auto-save active after first save"</span>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
