use leptos::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

/// Pixels to advance the prompter in one frame.
///
/// Scroll rate is **60 px/s per 1x speed**. This MUST stay consistent with
/// `prompter_view::estimated_remaining`, which computes time as
/// `remaining_px / (speed * 60)`. A regression here (e.g. the old `* 0.001`,
/// giving 1 px/s) makes the prompter look frozen.
pub fn scroll_delta_px(speed: f64, dt_ms: f64) -> f64 {
    speed * 0.06 * dt_ms
}

pub fn start_scroll_loop(
    is_playing: Signal<bool>,
    scroll_y: WriteSignal<f64>,
    speed: Signal<f64>,
) -> Rc<Cell<bool>> {
    let alive = Rc::new(Cell::new(true));
    let alive_cb = Rc::clone(&alive);

    let last_time = Rc::new(Cell::new(0.0));

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Default::default();
    let g = Rc::clone(&f);

    let callback = move |timestamp: f64| {
        if !alive_cb.get() {
            return;
        }
        if is_playing.get() {
            let dt = if last_time.get() == 0.0 {
                16.67
            } else {
                (timestamp - last_time.get()).min(50.0)
            };
            last_time.set(timestamp);
            let delta = scroll_delta_px(speed.get(), dt);
            scroll_y.update(|y| *y += delta);
        } else {
            last_time.set(0.0);
        }
        if let Some(closure) = g.borrow().as_ref() {
            let _ = window()
                .unwrap()
                .request_animation_frame(closure.as_ref().unchecked_ref());
        }
    };

    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(f64)>);
    *f.borrow_mut() = Some(closure);

    if let Some(closure) = f.borrow().as_ref() {
        let _ = window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref());
    }

    alive
}
