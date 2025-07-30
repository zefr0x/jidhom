use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;

/// Main navigation bar (should be visible in every page)
#[component]
pub fn NavBar() -> impl IntoView {
	let burger_state = RwSignal::new(false);

	view! {
		<nav class="navbar" role="navigation" aria-label="main navigation">

			<div class="navbar-brand">
				<A href="/" {..} class="navbar-item">
					// TODO: Add a logo rather than the name.
					<strong>{move_tr!("jidhom")}</strong>
				</A>

				<button
					aria-label="menu"
					aria-expanded=move || {burger_state().to_string()}
					class="navbar-burger"
					class:is-active=move || {burger_state()}
					on:click=move |_| {burger_state(!burger_state())}
				>
					<span aria-hidden="true"></span>
					<span aria-hidden="true"></span>
					<span aria-hidden="true"></span>
					<span aria-hidden="true"></span>
				</button>
			</div>

			<div class="navbar-menu" class:is-active=move || {burger_state()}>
				<div class="navbar-start">
				</div>

				<div class="navbar-end">
				</div>
			</div>
		</nav>
	}
}
