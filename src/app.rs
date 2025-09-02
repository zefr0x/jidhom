use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Html, Link, Stylesheet, Title, provide_meta_context};
use leptos_router::{
	components::{ProtectedRoute, Route, Router, Routes},
	path,
};

use crate::i18n;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	let client_cookies = components::utils::ClientCookies::new();
	provide_context(client_cookies);

	view! {
		<i18n::Provider>
			<Html {..} data-theme={move || if client_cookies.is_dark_theme() { "dark" } else { "light" }} />
			// VULN: We should compile tailwind locally or check the integrity of the source.
			<Link href="https://cdn.jsdelivr.net/npm/daisyui@5" rel="stylesheet" {..} type="text/css" />
			<Stylesheet id="leptos" href="/pkg/jidhom.css" />
			<Title text=move_tr!("jidhom") />

			<Router>
				<components::NavBar />

				<main>
					<Routes fallback=NotFound>
						<Route path=path!("/") view=pages::Home />

						// Those routes are disabled for active sessions
						<ProtectedRoute
							path=path!("/login")
							view=pages::LogIn
							condition=move || Some(!client_cookies.is_active_session())
							redirect_path=|| "/"
						/>
						<ProtectedRoute
							path=path!("/register")
							view=pages::ApplicatnRegister
							condition=move || Some(!client_cookies.is_active_session())
							redirect_path=|| "/"
						/>
					</Routes>
				</main>
			</Router>

			<script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
		</i18n::Provider>
	}
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
	// This can only be done during initial server-side rendering
	#[cfg(feature = "ssr")]
	{
		let resp = expect_context::<leptos_actix::ResponseOptions>();
		resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
	}

	view! { <h1>"Page Not Found!"</h1> }
}
