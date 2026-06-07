use openprompter_rs::app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| leptos::view! { <App/> });
}
