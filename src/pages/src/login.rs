use leptos::prelude::*;

/// The login page
#[component]
pub fn LogIn() -> impl IntoView {
	view! {
		<div class="flex items-center justify-center">
			<div class="hero">
				<div class="hero-content md:mt-24 sm:mt-12 mt-4 flex-col lg:flex-row-reverse">
					<components::UserAuthWidget />
				</div>
			</div>
		</div>
	}
}
