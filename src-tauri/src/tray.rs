//! System tray: pure menu-action mapping (unit-testable) + tray/window wiring.
//!
//! The window's close (X) button hides to the tray instead of quitting; the app
//! keeps running. The tray icon toggles window visibility on left-click and
//! exposes Show / Hide / Quit on right-click. Quit is the only full-exit path.
//!
//! `tray_action` is the testable seam — the rest is GUI runtime, verified
//! manually (`cargo tauri dev`).

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

/// Tray menu item ids. Kept as constants so the menu builder and the mapper
/// cannot drift apart.
pub const MENU_SHOW: &str = "show";
pub const MENU_HIDE: &str = "hide";
pub const MENU_QUIT: &str = "quit";

/// What a tray menu selection does to the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    Show,
    Hide,
    Quit,
}

/// Map a tray menu item id to an action. `None` for unknown ids (defensive — a
/// stray id is ignored rather than mis-dispatched).
pub fn tray_action(menu_id: &str) -> Option<TrayAction> {
    match menu_id {
        MENU_SHOW => Some(TrayAction::Show),
        MENU_HIDE => Some(TrayAction::Hide),
        MENU_QUIT => Some(TrayAction::Quit),
        _ => None,
    }
}

/// Window id of the single main window (matches tauri.conf.json / capabilities).
const MAIN_WINDOW: &str = "main";

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW) {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn hide_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW) {
        let _ = w.hide();
    }
}

fn toggle_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW) {
        if matches!(w.is_visible(), Ok(true)) {
            let _ = w.hide();
        } else {
            let _ = w.show();
            let _ = w.set_focus();
        }
    }
}

/// Apply a mapped [`TrayAction`] to the app.
pub fn apply_action(app: &AppHandle, action: TrayAction) {
    match action {
        TrayAction::Show => show_main(app),
        TrayAction::Hide => hide_main(app),
        TrayAction::Quit => app.exit(0),
    }
}

/// Build and register the system tray icon + menu. Called from `setup`.
pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItemBuilder::with_id(MENU_SHOW, "Show").build(app)?;
    let hide = MenuItemBuilder::with_id(MENU_HIDE, "Hide").build(app)?;
    let quit = MenuItemBuilder::with_id(MENU_QUIT, "Quit").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&show, &hide, &quit])
        .build()?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip("OpenPrompter RS")
        .menu(&menu)
        // Left-click toggles the window; the menu is right-click only.
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if let Some(action) = tray_action(event.id().as_ref()) {
                apply_action(app, action);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_ids() {
        assert_eq!(tray_action("show"), Some(TrayAction::Show));
        assert_eq!(tray_action("hide"), Some(TrayAction::Hide));
        assert_eq!(tray_action("quit"), Some(TrayAction::Quit));
    }

    #[test]
    fn unknown_id_is_none() {
        assert_eq!(tray_action("bogus"), None);
        assert_eq!(tray_action(""), None);
        assert_eq!(tray_action("SHOW"), None); // case-sensitive
    }

    #[test]
    fn constants_match_mapper() {
        assert_eq!(tray_action(MENU_SHOW), Some(TrayAction::Show));
        assert_eq!(tray_action(MENU_HIDE), Some(TrayAction::Hide));
        assert_eq!(tray_action(MENU_QUIT), Some(TrayAction::Quit));
    }
}
