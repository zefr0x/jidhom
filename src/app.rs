use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_meta::{Stylesheet, Title, provide_meta_context};

use crate::i18n;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<i18n::Provider>
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
