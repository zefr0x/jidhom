pub mod app;
mod i18n;

#[cfg(any(feature = "hydrate", feature = "csr"))]
pub fn init_client_logger() {
	use tracing_subscriber::prelude::*;

	// Show client panic messages in debug builds
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	// Initialize tracing subscriber
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::fmt::layer()
				.with_ansi(false)
				.with_level(false)
				.without_time()
				.with_writer(tracing_web::MakeWebConsoleWriter::new().with_pretty_level()),
		)
		.with(
			tracing_web::performance_layer()
				.with_details_from_fields(tracing_subscriber::fmt::format::Pretty::default()),
		)
		.init();
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	init_client_logger();

	leptos::mount::hydrate_lazy(app::App);
}
