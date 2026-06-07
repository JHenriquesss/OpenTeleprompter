use web_sys::KeyboardEvent;

pub type ShortcutAction = Box<dyn Fn() + 'static>;

pub fn handle_keydown(event: &KeyboardEvent, actions: &KeyboardActions) -> bool {
    let key = event.key();
    let shift = event.shift_key();
    let mut handled = true;

    match key.as_str() {
        " " => {
            event.prevent_default();
            (actions.toggle_play)();
        }
        "Escape" => (actions.exit_prompter)(),
        "f" | "F" => (actions.toggle_fullscreen)(),
        "r" | "R" => (actions.restart)(),
        "ArrowUp" => (actions.increase_speed)(),
        "ArrowDown" => (actions.decrease_speed)(),
        "ArrowLeft" => {
            if shift {
                (actions.jump_big_backward)();
            } else {
                (actions.jump_backward)();
            }
        }
        "ArrowRight" => {
            if shift {
                (actions.jump_big_forward)();
            } else {
                (actions.jump_forward)();
            }
        }
        "m" | "M" => (actions.toggle_mirror)(),
        "v" | "V" => (actions.toggle_mirror_vertical)(),
        "+" | "=" => (actions.increase_font_size)(),
        "-" | "_" => (actions.decrease_font_size)(),
        "h" | "H" => (actions.toggle_shortcut_help)(),
        _ => handled = false,
    }

    handled
}

pub struct KeyboardActions {
    pub toggle_play: ShortcutAction,
    pub exit_prompter: ShortcutAction,
    pub toggle_fullscreen: ShortcutAction,
    pub restart: ShortcutAction,
    pub increase_speed: ShortcutAction,
    pub decrease_speed: ShortcutAction,
    pub jump_backward: ShortcutAction,
    pub jump_forward: ShortcutAction,
    pub jump_big_backward: ShortcutAction,
    pub jump_big_forward: ShortcutAction,
    pub toggle_mirror: ShortcutAction,
    pub toggle_mirror_vertical: ShortcutAction,
    pub increase_font_size: ShortcutAction,
    pub decrease_font_size: ShortcutAction,
    pub toggle_shortcut_help: ShortcutAction,
}
