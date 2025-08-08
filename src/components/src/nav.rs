use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::components::A;
use phosphor_leptos::{Icon, IconWeight, USER_LIST};

/// Main navigation bar (should be visible in every page)
#[component]
pub fn NavBar() -> impl IntoView {
	let client_cookies = expect_context::<crate::utils::ClientCookies>();

	view! {
		<nav class="navbar bg-base-300 shadow-sm">
			<div class="navbar-start">
				<A href="/" {..} class="btn btn-ghost text-xl">
					// TODO: Add a logo rather than the name.
					<strong>{move_tr!("jidhom")}</strong>
				</A>
			</div>

			<div class="navbar-end">
				<Show
					when=move || client_cookies.is_active_session()
					fallback=|| view! {
						<A href="/login" {..}>
							<button class="btn btn-outline m-1">""{move_tr!("login")}</button>
						</A>
						<A href="/register" {..}>
							<button class="btn btn-outline btn-secondary m-1">{move_tr!("register")}</button>
						</A>
					}
				>
					<div class="dropdown">
						<div tabindex="0" role="button" class="btn btn-ghost">
							<i><Icon size="32px" weight=IconWeight::Fill icon=USER_LIST /></i>
						</div>
					</div>
				</Show>
			</div>
		</nav>
	}
}
