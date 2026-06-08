//! Component-level integration tests (Phase 11).
//!
//! Each test mounts a real Leptos component into a detached DOM node, wired to a
//! [`MockApi`] via context instead of the Tauri backend. This exercises the
//! component's real rendering + async data flow without `invoke`, native
//! dialogs, or a running Tauri shell.
//!
//! Tests split into two layers:
//! 1. **MockApi foundation** — verifies the test double behaves like the backend
//!    (pure async, no DOM, zero flake).
//! 2. **Mounted components** — verifies wiring (`use_context::<ApiCtx>`),
//!    rendering, and that components call through the abstraction.
//!
//! Async data settles via [`tick`]; we poll a bounded number of frames rather
//! than sleeping a fixed time, to stay non-flaky.

#![cfg(test)]

use std::rc::Rc;

use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use crate::bindings::mock_api::MockApi;
use crate::bindings::tauri_api::{ScriptData, ScriptPlaybackStateData, UpdateInfo};
use crate::bindings::{ApiCtx, AppApi};
use crate::components::prompter_view::PrompterView;
use crate::components::script_editor::ScriptEditor;
use crate::components::script_library::ScriptLibrary;
use crate::components::settings_panel::SettingsPanel;
use crate::components::update_banner::UpdateBanner;
use crate::state::app_state::AppState;
use crate::state::playback_state::PlaybackState;
use crate::state::toast::{ToastLevel, ToastState};
use crate::state::ui_state::UiState;

// ---- helpers -----------------------------------------------------------

fn mk_script(id: &str, title: &str, content: &str) -> ScriptData {
    ScriptData {
        id: id.to_string(),
        title: title.to_string(),
        content: content.to_string(),
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-01T00:00:00Z".to_string(),
    }
}

/// Yield to the event loop once (~10ms) so spawned async tasks and reactive
/// updates can flush before assertions.
async fn tick() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 10)
            .unwrap();
    });
    let _ = JsFuture::from(promise).await;
}

/// Let several rounds of chained async (e.g. settings → playback) settle.
async fn settle() {
    for _ in 0..6 {
        tick().await;
    }
}

/// Mount `view_fn` into a fresh detached div appended to the document body.
/// Returns the container so tests can inspect rendered text.
fn mount<F, V>(view_fn: F) -> web_sys::HtmlElement
where
    F: FnOnce() -> V + 'static,
    V: IntoView,
{
    let document = leptos::document();
    let div = document
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    document.body().unwrap().append_child(&div).unwrap();
    let container = div.clone();
    mount_to(div, view_fn);
    container
}

fn text_of(el: &web_sys::HtmlElement) -> String {
    el.text_content().unwrap_or_default()
}

// ---- prompter scroll math (freeze regression guard) --------------------

#[wasm_bindgen_test]
fn scroll_delta_is_60px_per_second_at_1x() {
    use crate::prompter::engine::scroll_delta_px;
    // 1x speed over one second must advance 60px — NOT 1px. The old engine
    // used `speed * 0.001` (1 px/s), which made the prompter look frozen.
    assert!((scroll_delta_px(1.0, 1000.0) - 60.0).abs() < 1e-9);
    // Linear in both speed and elapsed time.
    assert!((scroll_delta_px(2.0, 1000.0) - 120.0).abs() < 1e-9);
    assert!((scroll_delta_px(0.5, 500.0) - 15.0).abs() < 1e-9);
    // Zero elapsed time advances nothing.
    assert!(scroll_delta_px(5.0, 0.0).abs() < 1e-9);
}

// ---- layer 1: MockApi foundation ---------------------------------------

#[wasm_bindgen_test]
async fn mock_api_lists_and_searches_scripts() {
    let api = MockApi::new().with_scripts(vec![
        mk_script("1", "Keynote", "hello world"),
        mk_script("2", "Wedding Toast", "dearly beloved"),
    ]);

    let all = api.list_scripts().await.unwrap();
    assert_eq!(all.len(), 2);

    let hits = api.search_scripts("wedding").await.unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].title, "Wedding Toast");

    // search matches content too
    let by_content = api.search_scripts("beloved").await.unwrap();
    assert_eq!(by_content.len(), 1);

    assert!(api.was_called("list_scripts"));
    assert!(api.was_called("search_scripts"));
}

#[wasm_bindgen_test]
async fn mock_api_creates_deletes_duplicates() {
    let api = MockApi::new();
    let s = api.create_script("New", "body").await.unwrap();
    assert_eq!(api.script_count(), 1);

    let dup = api.duplicate_script(&s.id).await.unwrap();
    assert_eq!(api.script_count(), 2);
    assert!(dup.title.contains("(copy)"));

    api.delete_script(&s.id).await.unwrap();
    assert_eq!(api.script_count(), 1);
}

#[wasm_bindgen_test]
async fn mock_api_updates_and_resets_settings() {
    let api = MockApi::new();
    let mut s = api.get_settings().await.unwrap();
    assert_eq!(s.theme, "dark");

    s.theme = "light".to_string();
    s.font_size = 60.0;
    api.update_settings(&s).await.unwrap();
    assert_eq!(api.current_settings().theme, "light");

    let reset = api.reset_settings().await.unwrap();
    assert_eq!(reset.theme, "dark");
    assert_eq!(api.current_settings().font_size, 48.0);
}

#[wasm_bindgen_test]
async fn mock_api_saves_loads_clears_playback() {
    let api = MockApi::new();
    assert!(api.load_playback_state("1").await.unwrap().is_none());

    api.save_playback_state("1", 123.0, 1.5, Some(40.0), None, Some(true), None)
        .await
        .unwrap();
    let loaded = api.load_playback_state("1").await.unwrap().unwrap();
    assert_eq!(loaded.scroll_offset_px, 123.0);
    assert_eq!(loaded.speed_multiplier, 1.5);

    api.clear_playback_state("1").await.unwrap();
    assert!(api.load_playback_state("1").await.unwrap().is_none());
}

#[wasm_bindgen_test]
async fn mock_api_error_injection_fails_all() {
    let api = MockApi::new()
        .with_scripts(vec![mk_script("1", "X", "y")])
        .failing("boom");
    assert_eq!(api.list_scripts().await.err(), Some("boom".to_string()));
    assert_eq!(
        api.create_script("a", "b").await.err(),
        Some("boom".to_string())
    );
}

// ---- layer 2: mounted components ---------------------------------------

fn provide_base(api: ApiCtx, app_state: AppState) {
    provide_context::<ApiCtx>(api);
    provide_context(app_state);
    provide_context(PlaybackState::new());
    provide_context(UiState::new());
    provide_context(ToastState::new());
}

#[wasm_bindgen_test]
async fn script_library_renders_scripts_from_mock() {
    let mock = Rc::new(MockApi::new().with_scripts(vec![
        mk_script("1", "Keynote Speech", "..."),
        mk_script("2", "Birthday Toast", "..."),
    ]));
    let api: ApiCtx = mock.clone();

    let container = mount(move || {
        provide_base(api, AppState::new());
        view! { <ScriptLibrary /> }
    });
    settle().await;

    let txt = text_of(&container);
    assert!(txt.contains("Keynote Speech"), "got: {txt}");
    assert!(txt.contains("Birthday Toast"), "got: {txt}");
    assert!(mock.was_called("list_scripts"));
}

#[wasm_bindgen_test]
async fn script_library_shows_empty_state() {
    let mock = Rc::new(MockApi::new());
    let api: ApiCtx = mock.clone();

    let container = mount(move || {
        provide_base(api, AppState::new());
        view! { <ScriptLibrary /> }
    });
    settle().await;

    assert!(text_of(&container).contains("No scripts yet"));
}

#[wasm_bindgen_test]
async fn settings_panel_loads_through_mock() {
    let mock = Rc::new(MockApi::new());
    let api: ApiCtx = mock.clone();

    let container = mount(move || {
        provide_base(api, AppState::new());
        view! { <SettingsPanel /> }
    });
    settle().await;

    assert!(text_of(&container).contains("Settings"));
    assert!(mock.was_called("get_settings"));
}

#[wasm_bindgen_test]
async fn script_editor_loads_selected_script() {
    let mock =
        Rc::new(MockApi::new().with_scripts(vec![mk_script("42", "Loaded Title", "content")]));
    let api: ApiCtx = mock.clone();

    let app_state = AppState::new();
    app_state.editing_script_id.set(Some("42".to_string()));

    let _container = mount(move || {
        provide_base(api, app_state);
        view! { <ScriptEditor /> }
    });
    settle().await;

    assert!(mock.was_called("get_script"));
}

#[wasm_bindgen_test]
async fn prompter_view_shows_resume_dialog_when_state_exists() {
    let mock = Rc::new(
        MockApi::new()
            .with_scripts(vec![mk_script("7", "Talk", "line one line two")])
            .with_playback(ScriptPlaybackStateData {
                script_id: "7".to_string(),
                scroll_offset_px: 50.0,
                speed_multiplier: 1.0,
                font_size: Some(48.0),
                line_height: None,
                mirror_mode: Some(false),
                mirror_vertical: Some(false),
                updated_at: "2026-01-01".to_string(),
            }),
    );
    let api: ApiCtx = mock.clone();

    let app_state = AppState::new();
    app_state.selected_script_id.set(Some("7".to_string()));

    let container = mount(move || {
        provide_base(api, app_state);
        view! { <PrompterView /> }
    });
    settle().await;

    assert!(mock.was_called("load_playback_state"));
    assert!(
        text_of(&container).contains("Resume"),
        "resume dialog missing: {}",
        text_of(&container)
    );
}

// ---- Phase 12: import/export/duplicate/delete flows ---------------------
//
// These exercise the real ScriptLibrary action handlers by dispatching DOM
// clicks on the rendered buttons, driven by MockApi. They reference test
// support that does not exist yet (RED): MockApi::fail_on / call_count /
// was_not_called / exported / scripts, ToastState::snapshot, and the
// ConfirmModal aria-labels. Implementation lands in the green step.

/// Mount `<ScriptLibrary>` with the given mock API and toast state, returning
/// the container. `toast` is `Copy`, so the caller keeps a handle for asserts.
fn mount_library(api: ApiCtx, toast: ToastState) -> web_sys::HtmlElement {
    mount(move || {
        provide_context::<ApiCtx>(api);
        provide_context(AppState::new());
        provide_context(PlaybackState::new());
        provide_context(UiState::new());
        provide_context(toast);
        view! { <ScriptLibrary /> }
    })
}

fn query_click(container: &web_sys::HtmlElement, selector: &str) -> bool {
    match container.query_selector(selector) {
        Ok(Some(el)) => {
            let btn: web_sys::HtmlElement = el.dyn_into().unwrap();
            btn.click();
            true
        }
        _ => false,
    }
}

/// Click a button by its `title` attribute (row action buttons).
fn click_by_title(container: &web_sys::HtmlElement, title: &str) -> bool {
    query_click(container, &format!("[title=\"{title}\"]"))
}

/// Click a button by its `aria-label` (modal confirm/cancel).
fn click_by_aria(container: &web_sys::HtmlElement, label: &str) -> bool {
    query_click(container, &format!("[aria-label=\"{label}\"]"))
}

/// Click the first `<button>` whose trimmed text matches `label` (e.g. Import).
fn click_text(container: &web_sys::HtmlElement, label: &str) -> bool {
    let buttons = container.query_selector_all("button").unwrap();
    for i in 0..buttons.length() {
        let node = buttons.get(i).unwrap();
        let el: web_sys::HtmlElement = node.dyn_into().unwrap();
        if el.text_content().unwrap_or_default().trim() == label {
            el.click();
            return true;
        }
    }
    false
}

fn assert_toast_contains_success(toast: &ToastState) {
    assert!(
        toast
            .snapshot()
            .iter()
            .any(|t| matches!(t.level, ToastLevel::Success)),
        "expected a success toast, got: {:?}",
        toast.snapshot()
    );
}

fn assert_toast_contains_error(toast: &ToastState) {
    assert!(
        toast
            .snapshot()
            .iter()
            .any(|t| matches!(t.level, ToastLevel::Error)),
        "expected an error toast, got: {:?}",
        toast.snapshot()
    );
}

fn assert_no_error_toast(toast: &ToastState) {
    assert!(
        !toast
            .snapshot()
            .iter()
            .any(|t| matches!(t.level, ToastLevel::Error)),
        "expected no error toast, got: {:?}",
        toast.snapshot()
    );
}

// ---- positive --------------------------------------------------------------

#[wasm_bindgen_test]
async fn import_success_creates_script_with_content() {
    let path = "C:\\scripts\\my_talk.txt";
    let mock = Rc::new(
        MockApi::new()
            .with_open_dialog(path)
            .with_file(path, "Hello from import"),
    );
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(click_text(&container, "Import"), "Import button not found");
    settle().await;

    assert_eq!(mock.call_count("import_script_from_txt"), 1);
    let scripts = mock.scripts();
    let imported = scripts
        .iter()
        .find(|s| s.title == "my_talk")
        .expect("imported script with filename-derived title missing");
    assert_eq!(imported.content, "Hello from import");
    assert_toast_contains_success(&toast);
}

#[wasm_bindgen_test]
async fn export_success_exports_correct_script() {
    let out = "C:\\out\\talk.txt";
    let mock = Rc::new(
        MockApi::new()
            .with_scripts(vec![mk_script("7", "Doomed Export", "body")])
            .with_save_dialog(out),
    );
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(
        click_by_title(&container, "Export"),
        "Export button missing"
    );
    settle().await;

    let exported = mock.exported();
    assert_eq!(exported.len(), 1);
    assert_eq!(exported[0].0, "7", "wrong script id exported");
    assert_eq!(exported[0].1, out);
    assert_toast_contains_success(&toast);
}

#[wasm_bindgen_test]
async fn duplicate_creates_copy() {
    let mock = Rc::new(MockApi::new().with_scripts(vec![mk_script("3", "Original", "x")]));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(
        click_by_title(&container, "Duplicate"),
        "Duplicate button missing"
    );
    settle().await;

    assert_eq!(mock.call_count("duplicate_script"), 1);
    assert_eq!(mock.script_count(), 2);
    assert_toast_contains_success(&toast);
}

#[wasm_bindgen_test]
async fn delete_confirm_full_sequence() {
    let mock = Rc::new(MockApi::new().with_scripts(vec![mk_script("9", "Doomed Row", "x")]));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;
    assert!(text_of(&container).contains("Doomed Row"));

    // open the confirmation modal
    assert!(click_by_title(&container, "Delete"), "row Delete missing");
    settle().await;
    // delete must NOT happen before confirmation
    assert!(
        mock.was_not_called("delete_script"),
        "delete fired before confirmation"
    );

    // confirm
    assert!(
        click_by_aria(&container, "Confirm"),
        "modal Confirm button missing"
    );
    settle().await;

    assert_eq!(
        mock.call_count("delete_script"),
        1,
        "delete should fire exactly once after confirm"
    );
    assert!(
        !text_of(&container).contains("Doomed Row"),
        "row not removed"
    );
    assert_toast_contains_success(&toast);
}

// ---- negative --------------------------------------------------------------

#[wasm_bindgen_test]
async fn delete_cancel_keeps_row() {
    let mock = Rc::new(MockApi::new().with_scripts(vec![mk_script("9", "Survivor", "x")]));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(click_by_title(&container, "Delete"), "row Delete missing");
    settle().await;
    assert!(click_by_aria(&container, "Cancel"), "modal Cancel missing");
    settle().await;

    assert!(mock.was_not_called("delete_script"));
    assert!(text_of(&container).contains("Survivor"), "row was removed");
    assert_no_error_toast(&toast);
}

#[wasm_bindgen_test]
async fn import_cancel_does_nothing() {
    // No with_open_dialog -> open_file_dialog returns None (user cancelled)
    let mock = Rc::new(MockApi::new());
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(click_text(&container, "Import"), "Import button not found");
    settle().await;

    assert!(mock.was_not_called("import_script_from_txt"));
    assert_eq!(mock.script_count(), 0);
    assert_no_error_toast(&toast);
}

#[wasm_bindgen_test]
async fn import_failure_shows_error_toast() {
    let path = "C:\\scripts\\bad.txt";
    let mock = Rc::new(
        MockApi::new()
            .with_open_dialog(path)
            .with_file(path, "content")
            .fail_on("import_script_from_txt"),
    );
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(click_text(&container, "Import"), "Import button not found");
    settle().await;

    // dialog + read succeed; only the import command fails
    assert_eq!(mock.call_count("import_script_from_txt"), 1);
    assert_eq!(mock.script_count(), 0);
    assert_toast_contains_error(&toast);
}

#[wasm_bindgen_test]
async fn export_cancel_does_not_export() {
    // No with_save_dialog -> save_file_dialog returns None
    let mock = Rc::new(MockApi::new().with_scripts(vec![mk_script("7", "Keep", "x")]));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(
        click_by_title(&container, "Export"),
        "Export button missing"
    );
    settle().await;

    assert!(mock.was_not_called("export_script_to_txt_file"));
    assert!(mock.exported().is_empty());
    assert_no_error_toast(&toast);
}

#[wasm_bindgen_test]
async fn delete_failure_keeps_row_and_errors() {
    let mock = Rc::new(
        MockApi::new()
            .with_scripts(vec![mk_script("9", "Sticky Row", "x")])
            .fail_on("delete_script"),
    );
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_library(api, toast);
    settle().await;

    assert!(click_by_title(&container, "Delete"), "row Delete missing");
    settle().await;
    assert!(
        click_by_aria(&container, "Confirm"),
        "modal Confirm missing"
    );
    settle().await;

    assert_eq!(mock.call_count("delete_script"), 1);
    assert!(
        text_of(&container).contains("Sticky Row"),
        "row was removed"
    );
    assert_toast_contains_error(&toast);
}

// ---- Phase 14: updater (check / prompt / install) ----------------------
//
// UpdateBanner auto-checks for an update on mount through AppApi. If an update
// is available it renders a prompt with Install / Dismiss. These reference test
// support that does not exist yet (RED): UpdateInfo, MockApi::with_update,
// and the UpdateBanner component itself.

fn mk_update(version: &str) -> UpdateInfo {
    UpdateInfo {
        version: version.to_string(),
        current_version: "0.10.0".to_string(),
        notes: Some(format!("Release notes for {version}")),
        date: Some("2026-06-01".to_string()),
    }
}

/// Mount `<UpdateBanner>` with the given mock API + toast state.
fn mount_banner(api: ApiCtx, toast: ToastState) -> web_sys::HtmlElement {
    mount(move || {
        provide_context::<ApiCtx>(api);
        provide_context(toast);
        view! { <UpdateBanner /> }
    })
}

#[wasm_bindgen_test]
async fn update_available_shows_prompt_with_version() {
    let mock = Rc::new(MockApi::new().with_update(mk_update("0.11.0")));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_banner(api, toast);
    settle().await;

    assert!(mock.was_called("check_for_update"));
    let txt = text_of(&container);
    assert!(txt.contains("0.11.0"), "banner missing new version: {txt}");
    assert!(
        click_text(&container, "Install") || click_by_aria(&container, "Install update"),
        "Install button missing: {txt}"
    );
}

#[wasm_bindgen_test]
async fn update_install_success_calls_install() {
    let mock = Rc::new(MockApi::new().with_update(mk_update("0.11.0")));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_banner(api, toast);
    settle().await;

    assert!(
        click_by_aria(&container, "Install update") || click_text(&container, "Install"),
        "Install button missing"
    );
    settle().await;

    assert_eq!(mock.call_count("install_update"), 1);
    assert_no_error_toast(&toast);
}

#[wasm_bindgen_test]
async fn no_update_shows_no_prompt() {
    let mock = Rc::new(MockApi::new()); // with_update never set -> None
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_banner(api, toast);
    settle().await;

    assert!(mock.was_called("check_for_update"));
    let txt = text_of(&container);
    assert!(
        !txt.contains("Install") && !txt.contains("available"),
        "unexpected update prompt when no update: {txt}"
    );
    assert!(mock.was_not_called("install_update"));
    assert_no_error_toast(&toast);
}

#[wasm_bindgen_test]
async fn update_check_failure_shows_error_toast() {
    let mock = Rc::new(MockApi::new().fail_on("check_for_update"));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let _container = mount_banner(api, toast);
    settle().await;

    assert!(mock.was_called("check_for_update"));
    assert_toast_contains_error(&toast);
}

#[wasm_bindgen_test]
async fn update_install_failure_shows_error_toast() {
    let mock = Rc::new(
        MockApi::new()
            .with_update(mk_update("0.11.0"))
            .fail_on("install_update"),
    );
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_banner(api, toast);
    settle().await;

    assert!(
        click_by_aria(&container, "Install update") || click_text(&container, "Install"),
        "Install button missing"
    );
    settle().await;

    assert_eq!(mock.call_count("install_update"), 1);
    assert_toast_contains_error(&toast);
}

#[wasm_bindgen_test]
async fn update_dismiss_hides_prompt() {
    let mock = Rc::new(MockApi::new().with_update(mk_update("0.11.0")));
    let api: ApiCtx = mock.clone();
    let toast = ToastState::new();

    let container = mount_banner(api, toast);
    settle().await;
    assert!(text_of(&container).contains("0.11.0"));

    assert!(
        click_by_aria(&container, "Dismiss update") || click_text(&container, "Dismiss"),
        "Dismiss button missing"
    );
    settle().await;

    assert!(
        !text_of(&container).contains("0.11.0"),
        "prompt still visible after dismiss"
    );
    assert!(mock.was_not_called("install_update"));
}
