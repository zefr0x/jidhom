use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/jidhom.css" />
		<Title text="Jidhom" />
	}
}
