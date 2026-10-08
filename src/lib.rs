pub mod app;
pub mod charts_svg;
pub mod components;
pub mod credits;
pub mod date;
pub mod icons;
pub mod models;
pub mod persist;
pub mod sections;
pub mod store;

#[cfg(target_arch = "wasm32")]
pub fn mount() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
