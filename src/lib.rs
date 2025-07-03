pub mod app;
#[cfg(feature = "ssr")]
pub mod db;
mod i18n;
mod utils;

#[cfg(any(feature = "hydrate", feature = "csr"))]
pub fn init_client_logger() {
	// Show client panic messages in debug builds
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	// Initialize logger
	#[cfg(not(debug_assertions))]
	console_log::init().unwrap();
	// TODO: Is there a way to configure logging level in the client?
	#[cfg(debug_assertions)]
	console_log::init_with_level(log::Level::Trace).unwrap();
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	init_client_logger();

	leptos::mount::hydrate_body(app::App);
}
