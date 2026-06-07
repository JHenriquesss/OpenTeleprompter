pub mod app_api;
pub mod tauri_api;

#[cfg(test)]
pub mod mock_api;

use std::rc::Rc;

pub use app_api::{AppApi, RealTauriApi};

/// Leptos context handle for the frontend API.
///
/// Production provides `Rc::new(RealTauriApi)`; component tests provide
/// `Rc::new(MockApi::...)`. Components fetch it with
/// `use_context::<ApiCtx>()`.
pub type ApiCtx = Rc<dyn AppApi>;
