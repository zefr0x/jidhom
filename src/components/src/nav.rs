use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;

/// Main navigation bar (should be visible in every page)
#[component]
pub fn NavBar() -> impl IntoView {
	view! {
		<nav class="navbar bg-base-300 shadow-sm">
			<div class="navbar-start">
				<A href="/" {..} class="btn btn-ghost text-xl">
					// TODO: Add a logo rather than the name.
					<strong>{move_tr!("jidhom")}</strong>
				</A>
			</div>
		</nav>
	}
}
