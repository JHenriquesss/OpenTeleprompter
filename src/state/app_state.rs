use leptos::{RwSignal, SignalUpdate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
    Library,
    Editor,
    Prompter,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub view: RwSignal<View>,
    pub selected_script_id: RwSignal<Option<String>>,
    pub editing_script_id: RwSignal<Option<String>>,
    pub search_query: RwSignal<String>,
    pub library_refresh_trigger: RwSignal<u32>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            view: RwSignal::new(View::Library),
            selected_script_id: RwSignal::new(None),
            editing_script_id: RwSignal::new(None),
            search_query: RwSignal::new(String::new()),
            library_refresh_trigger: RwSignal::new(0),
        }
    }

    pub fn refresh_library(&self) {
        self.library_refresh_trigger.update(|t| *t += 1);
    }
}
