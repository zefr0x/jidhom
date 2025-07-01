//! The entry point of Jidhom.

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	use actix_files::Files;
	use actix_web::{App, HttpServer, middleware, web};
	use anyhow::Context;
	use leptos::{config::get_configuration, logging::log, prelude::*};
	use leptos_actix::{LeptosRoutes, generate_route_list};
	use leptos_meta::MetaTags;

	use jidhom::db::Connection;

	// Read environment variables
	#[cfg(debug_assertions)]
	#[expect(unused_must_use)]
	dotenvy::dotenv();

	let database_url = std::env::var("DATABASE_URL").context("you must have `DATABASE_URL` set")?;

	// Create a database connections pool
	let database_connection = web::ThinData(Connection::new(&database_url).await?);

	let conf = get_configuration(None)?;
	let addr = conf.leptos_options.site_addr;

	HttpServer::new(move || {
		// Generate list of Leptos App routes
		let routes = generate_route_list(jidhom::app::App);
		let leptos_options = &conf.leptos_options;
		let site_root = leptos_options.site_root.clone().to_string();

		log!("listening on http://{}", &addr);

		App::new()
			// Serve JS/WASM/CSS from `pkg`
			.service(Files::new("/pkg", format!("{site_root}/pkg")))
			// Serve other assets from the `assets` directory
			.service(Files::new("/assets", &site_root))
			.leptos_routes(routes, {
				let leptos_options = leptos_options.clone();
				move || {
					view! {
						<!DOCTYPE html>
						<html lang="en">
							<head>
								<meta charset="utf-8" />
								<meta name="viewport" content="width=device-width, initial-scale=1" />
								<AutoReload options=leptos_options.clone() />
								<HydrationScripts options=leptos_options.clone() />
								<MetaTags />
							</head>
							<body>
								<jidhom::app::App />
							</body>
						</html>
					}
				}
			})
			.app_data(web::Data::new(leptos_options.to_owned()))
			.app_data(database_connection.clone())
			.wrap(middleware::Compress::default())
	})
	.bind(&addr)?
	.run()
	.await?;

	Ok(())
}

// Client-side main function for e.g. usage with `trunk serve`
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	leptos::mount::mount_to_body(jidhom::app::App);
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
	unreachable!()
}
