#![warn(clippy::all)]

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod app;
pub mod bindings;
pub mod components;
pub mod prompter;
pub mod state;

#[cfg(test)]
mod component_tests;
