use leptos::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

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
            let px_per_ms = speed.get() * 0.001;
            scroll_y.update(|y| *y += px_per_ms * dt);
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
