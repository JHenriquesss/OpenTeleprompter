use crate::bindings::tauri_api;
use crate::components::confirm_modal::ConfirmModal;
use crate::state::app_state::{AppState, View};
use crate::state::toast::ToastState;
use crate::state::ui_state::UiState;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn ScriptLibrary() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not provided");
    let toast = expect_context::<ToastState>();

    let search_ref = create_node_ref::<Input>();

    let on_search = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(input) = search_ref.get() {
                let query = input.value();
                app_state.search_query.set(query);
            }
        }
    };

    let searched_scripts = create_resource(
        move || {
            (
                app_state.search_query.get(),
                app_state.library_refresh_trigger.get(),
            )
        },
        move |(query, _)| async move {
            if query.is_empty() {
                tauri_api::list_scripts().await
            } else {
                tauri_api::search_scripts(&query).await
            }
        },
    );

    let ui = use_context::<UiState>().expect("UiState not provided");

    let on_select = move |id: String| {
        app_state.selected_script_id.set(Some(id));
    };

    let on_start_prompter = move |id: String| {
        ui.rehearse_mode.set(false);
        app_state.selected_script_id.set(Some(id));
        app_state.view.set(View::Prompter);
    };

    let on_rehearse = move |id: String| {
        ui.rehearse_mode.set(true);
        app_state.selected_script_id.set(Some(id));
        app_state.view.set(View::Prompter);
    };

    let on_edit = move |id: String| {
        app_state.selected_script_id.set(Some(id.clone()));
        app_state.editing_script_id.set(Some(id));
        app_state.view.set(View::Editor);
    };

    let show_delete_modal = create_rw_signal(false);
    let pending_delete_id = create_rw_signal::<Option<String>>(None);

    let on_delete = move |id: String| {
        pending_delete_id.set(Some(id));
        show_delete_modal.set(true);
    };

    let confirm_delete = move || {
        if let Some(id) = pending_delete_id.get() {
            let toast = toast.clone();
            spawn_local(async move {
                match tauri_api::delete_script(&id).await {
                    Ok(_) => {
                        toast.add_success("Script deleted");
                        app_state.refresh_library();
                    }
                    Err(e) => toast.add_error(&format!("Delete failed: {}", e)),
                }
            });
        }
    };

    let on_duplicate = move |id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            match tauri_api::duplicate_script(&id).await {
                Ok(script) => {
                    toast.add_success("Script duplicated");
                    app_state.selected_script_id.set(Some(script.id.clone()));
                    app_state.editing_script_id.set(Some(script.id));
                    app_state.view.set(View::Editor);
                    app_state.refresh_library();
                }
                Err(e) => toast.add_error(&format!("Duplicate failed: {}", e)),
            }
        });
    };

    let on_new_script = move |_| {
        spawn_local(async move {
            if let Ok(script) = tauri_api::create_script("New Script", "").await {
                app_state.selected_script_id.set(Some(script.id.clone()));
                app_state.editing_script_id.set(Some(script.id));
                app_state.view.set(View::Editor);
                app_state.refresh_library();
            }
        });
    };

    let on_import = move |_| {
        let toast = toast.clone();
        spawn_local(async move {
            match tauri_api::open_file_dialog().await {
                Ok(Some(path)) => match tauri_api::read_text_file(&path).await {
                    Ok(Some(content)) => {
                        let file_name = path
                            .rsplit('\\')
                            .next()
                            .unwrap_or("imported.txt")
                            .to_string();
                        match tauri_api::import_script_from_txt(&content, &file_name).await {
                            Ok(script) => {
                                toast.add_success("Script imported");
                                app_state.selected_script_id.set(Some(script.id.clone()));
                                app_state.editing_script_id.set(Some(script.id));
                                app_state.view.set(View::Editor);
                                app_state.refresh_library();
                            }
                            Err(e) => toast.add_error(&format!("Import failed: {}", e)),
                        }
                    }
                    Ok(None) => {}
                    Err(e) => toast.add_error(&format!("Read file failed: {}", e)),
                },
                Ok(None) => {}
                Err(e) => toast.add_error(&format!("Open dialog failed: {}", e)),
            }
        });
    };

    let on_export = move |id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            match tauri_api::save_file_dialog().await {
                Ok(Some(path)) => match tauri_api::export_script_to_txt_file(&id, &path).await {
                    Ok(_) => toast.add_success("Script exported"),
                    Err(e) => toast.add_error(&format!("Export failed: {}", e)),
                },
                Ok(None) => {}
                Err(e) => toast.add_error(&format!("Save dialog failed: {}", e)),
            }
        });
    };

    view! {
        <div style="padding: 24px; height: 100%; display: flex; flex-direction: column;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                <h1 style="font-size: 24px; color: var(--text-main); margin: 0;">Script Library</h1>
                <div style="display: flex; gap: 8px;">
                    <button
                        on:click=on_new_script
                        style="
                            padding: 8px 16px; border: none; border-radius: 6px;
                            background: var(--button-primary-bg); color: var(--button-primary-text); cursor: pointer;
                            font-size: 14px;
                        "
                    >
                        + New Script
                    </button>
                    <button
                        on:click=move |_| on_import(())
                        style="
                            padding: 8px 16px; border: 1px solid var(--button-ghost-border); border-radius: 6px;
                            background: transparent; color: var(--button-ghost-text); cursor: pointer;
                            font-size: 14px;
                        "
                    >
                        Import
                    </button>
                </div>
            </div>

            <input
                node_ref=search_ref
                on:keydown=on_search
                placeholder="Search scripts (press Enter)..."
                style="
                    width: 100%; padding: 10px 14px; margin-bottom: 16px;
                    border: 1px solid var(--input-border); border-radius: 8px;
                    background: var(--input-bg); color: var(--input-text);
                    font-size: 14px; outline: none;
                "
            />

            <div style="flex: 1; overflow-y: auto;">
                {move || match searched_scripts.get() {
                    None => view! {
                        <div style="color: var(--text-loading); text-align: center; padding: 48px;">
                            <div style="font-size: 32px; margin-bottom: 12px;">{"📖"}</div>
                            <div>"Loading scripts..."</div>
                            <div style="font-size: 12px; color: var(--text-dim); margin-top: 8px;">
                                "Fetching from local storage"
                            </div>
                        </div>
                    }.into_view(),
                    Some(Err(e)) => view! {
                        <div style="color: var(--danger-text); padding: 16px; text-align: center;">
                            <div style="font-size: 32px; margin-bottom: 12px;">{"⚠️"}</div>
                            <div>"Error: " {e}</div>
                        </div>
                    }.into_view(),
                    Some(Ok(list)) => {
                        let is_searching = !app_state.search_query.get().is_empty();
                        if list.is_empty() {
                            if is_searching {
                                view! {
                                    <div style="color: var(--text-loading); text-align: center; padding: 48px;">
                                        <div style="font-size: 32px; margin-bottom: 12px;">{"🔍"}</div>
                                        <div>"No scripts match your search."</div>
                                        <div style="font-size: 12px; color: var(--text-muted2); margin-top: 8px;">
                                            "Try a different search term."
                                        </div>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div style="color: var(--text-loading); text-align: center; padding: 48px;">
                                        <div style="font-size: 32px; margin-bottom: 12px;">{"📝"}</div>
                                        <div>"No scripts yet."</div>
                                        <div style="font-size: 13px; color: var(--text-muted2); margin-top: 8px;">
                                            "Create a new script or import a text file to get started."
                                        </div>
                                        <button
                                            on:click=on_new_script
                                            style="
                                                margin-top: 16px; padding: 10px 24px; border: none;
                                                border-radius: 6px; background: var(--button-primary-bg); color: var(--button-primary-text);
                                                cursor: pointer; font-size: 14px;
                                            "
                                        >
                                            + New Script
                                        </button>
                                    </div>
                                }.into_view()
                            }
                        } else {
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    {list.into_iter().map(|script| {
                                        let sid = script.id.clone();
                                        let sid_play = script.id.clone();
                                        let sid_rehearse = script.id.clone();
                                        let sid_edit = script.id.clone();
                                        let sid_dup = script.id.clone();
                                        let sid_del = script.id.clone();
                                        let sid_export = script.id.clone();
                                        view! {
                                            <div
                                                style="
                                                    background: var(--card-bg);
                                                    border-radius: 8px;
                                                    padding: 14px 18px;
                                                    cursor: pointer;
                                                    display: flex;
                                                    justify-content: space-between;
                                                    align-items: center;
                                                    transition: background 0.2s;
                                                    border: 1px solid var(--card-border);
                                                "
                                            >
                                                <div
                                                    on:click=move |_| on_select(sid.clone())
                                                    style="flex: 1;"
                                                >
                                                    <div style="font-size: 16px; font-weight: 500; color: var(--card-text);">
                                                        {&script.title}
                                                    </div>
                                                    <div style="font-size: 12px; color: var(--text-muted2); margin-top: 4px;">
                                                        {script.updated_at[..script.updated_at.len().min(10)].to_string()}
                                                    </div>
                                                </div>
                                                <div style="display: flex; gap: 6px;">
                                                    <button
                                                        on:click=move |_| on_start_prompter(sid_play.clone())
                                                        title="Play"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"▶"}
                                                    </button>
                                                    <button
                                                        on:click=move |_| on_rehearse(sid_rehearse.clone())
                                                        title="Rehearse"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"🎤"}
                                                    </button>
                                                    <button
                                                        on:click=move |_| on_edit(sid_edit.clone())
                                                        title="Edit"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"✏️"}
                                                    </button>
                                                    <button
                                                        on:click=move |_| on_duplicate(sid_dup.clone())
                                                        title="Duplicate"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"📄"}
                                                    </button>
                                                    <button
                                                        on:click=move |_| on_export(sid_export.clone())
                                                        title="Export"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--button-secondary-bg); color: var(--button-secondary-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"💾"}
                                                    </button>
                                                    <button
                                                        on:click=move |_| on_delete(sid_del.clone())
                                                        title="Delete"
                                                        style="padding: 4px 10px; border: none; border-radius: 4px; background: var(--danger-bg); color: var(--danger-text); cursor: pointer; font-size: 12px;"
                                                    >
                                                        {"🗑️"}
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_view()
                        }
                    }
                }}
            </div>
        </div>

        <ConfirmModal
            show=show_delete_modal
            title="Delete Script".to_string()
            message="Are you sure you want to delete this script? This action cannot be undone.".to_string()
            confirm_label="Delete".to_string()
            on_confirm=confirm_delete
        />
    }
}
