use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Link, Stylesheet, Title, provide_meta_context};

use crate::i18n;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<i18n::Provider>
			<Link
				rel="stylesheet"
				href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css"
				integrity="sha512-yh2RE0wZCVZeysGiqTwDTO/dKelCbS9bP2L94UvOFtl/FKXcNAje3Y2oBg/ZMZ3LS1sicYk4dYVGtDex75fvvA=="
				crossorigin="anonymous"
			/>
			<Stylesheet id="leptos" href="/pkg/jidhom.css" />
			<Title text=move_tr!("jidhom") />
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
