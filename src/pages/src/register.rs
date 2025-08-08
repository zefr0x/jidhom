use leptos::prelude::*;
use leptos_fluent::move_tr;

/// The login page
#[component]
pub fn ApplicatnRegister() -> impl IntoView {
	view! {
		<div class="flex items-center justify-center">
			<div class="hero">
				<div class="hero-content md:mt-24 sm:mt-12 mt-4 flex-col md:flex-row-reverse">
					<div class="text-center md:text-left xs:w-xs sm:w-sm lg:w-lg xl:w-xl">
						<ul class="steps md:steps-vertical">
							<li class="step step-primary">{move_tr!("register")}</li>
							<li class="step">{move_tr!("create-profile")}</li>
							<li class="step">{move_tr!("apply")}</li>
						</ul>
					</div>

					<components::ApplicantRegisterWidget />
				</div>
			</div>
		</div>
	}
}
