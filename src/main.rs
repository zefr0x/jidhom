//! The entry point of Jidhom.

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	use actix_files::Files;
	use actix_web::{App, HttpServer, middleware, web};
	use anyhow::Context;
	use leptos::{config::get_configuration, prelude::*};
	use leptos_actix::{LeptosRoutes, generate_route_list};
	use leptos_meta::MetaTags;

	// Load `.env` file in debug builds
	#[cfg(debug_assertions)]
	#[expect(unused_must_use)]
	dotenvy::dotenv();

	// Initialize tracing subscriber with a non-blocking writer
	let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stderr());
	tracing_subscriber::fmt()
		.with_span_events(
			tracing_subscriber::fmt::format::FmtSpan::NEW | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
		)
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.with_writer(non_blocking_writer)
		.init();

	// Load environment variables
	let database_url = std::env::var("DATABASE_URL").context("you must have `DATABASE_URL` set")?;

	// Create a database connections pool
	let database_connection = web::ThinData(db::Connection::<db::NoneState>::new(&database_url).await?);

	let conf = get_configuration(None)?;
	let addr = conf.leptos_options.site_addr;

	HttpServer::new(move || {
		// Generate list of Leptos App routes
		let routes = generate_route_list(jidhom::app::App);
		let leptos_options = &conf.leptos_options;
		let site_root = leptos_options.site_root.clone().to_string();

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
			// A tracing span for every request
			.wrap({
				struct DomainRootSpanBuilder;

				impl tracing_actix_web::RootSpanBuilder for DomainRootSpanBuilder {
					fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
						let root_span = tracing_actix_web::root_span!(
							request,
							cookie.session.id = tracing::field::Empty,
							cookie.lang = tracing::field::Empty
						);

						// TODO: Consider that for privacy we don't need to log session.id of a request that doesn't need authorization.
						if let Some(session_id) = request.cookie("id") {
							root_span.record("cookie.session.id", session_id.value());
						}
						if let Some(lang) = request.cookie("lang") {
							root_span.record("cookie.lang", lang.value());
						}

						root_span
					}

					fn on_request_end<B: actix_web::body::MessageBody>(
						span: tracing::Span,
						outcome: &Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
					) {
						tracing_actix_web::DefaultRootSpanBuilder::on_request_end(span, outcome);
					}
				}

				tracing_actix_web::TracingLogger::<DomainRootSpanBuilder>::new()
			})
	})
	.bind(&addr)?
	.run()
	.await?;

	Ok(())
}

// Client-side main function for e.g. usage with `trunk serve`
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
	jidhom::init_client_logger();

	leptos::mount::mount_to_body(jidhom::app::App);
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
	unreachable!()
}
