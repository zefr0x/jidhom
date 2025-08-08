use leptos::prelude::*;
use leptos_fluent::move_tr;
use phosphor_leptos::{Icon, IconWeight, X_CIRCLE};

/// User login/authentication widget
#[component]
pub fn UserAuthWidget() -> impl IntoView {
	let client_cookies = expect_context::<crate::utils::ClientCookies>();

	let create_new_session = ServerAction::<api::authentication::CreateNewSession>::new();
	let auth_result = create_new_session.value();
	let auth_pending = create_new_session.pending();

	#[expect(unused_results, reason = "No need")]
	Effect::new(move || {
		// NOTE: Redirection to homepage will be done automatically, since `login` route is protected from active sessions.
		if let Some(Ok(auth_result)) = *auth_result.read()
			&& auth_result
		{
			// HACK: `use_cookie`'s signal will not be triggered with a Set-Cookie HTTP header, so we need to trigger it manually.
			client_cookies.set_is_active_session(true);
		}
	});

	view! {
		<div class="card bg-base-100 w-full max-w-sm shrink-0 shadow-2xl">
			<div class="card-body">
				<ActionForm action=create_new_session>
					<fieldset class="fieldset">
						<legend class="fieldset-legend">{move_tr!("login")}</legend>

						<label class="floating-label">
							<span>{move_tr!("user-identifier")}</span>

							<input
								name="user_id"
								class="input validator"
								required
								placeholder={move_tr!("user-identifier")}
							/>

							<p class="validator-hint"></p>
						</label>

						<label class="floating-label">
							<span>{move_tr!("password")}</span>

							<input
								type="password"
								name="user_password"
								class="input validator"
								required
								placeholder={move_tr!("password")}
							/>

							<p class="validator-hint"></p>
						</label>

						<Show when=move || auth_result.get().is_some_and(|x| x.is_ok_and(|x| !x))>
							<div role="alert" class="alert alert-error">
								<i><Icon weight=IconWeight::Fill icon=X_CIRCLE /></i>
								<span>{move_tr!("authentication-failed")}</span>
							</div>
						</Show>

						<button type="submit" class="btn btn-neutral mt-4" class:btn-disabled=move || auth_pending.get()>
							<Show when=move || auth_pending.get()>
								<span class="loading loading-spinner"></span>
							</Show>
							{move_tr!("login")}
						</button>
					</fieldset>
				</ActionForm>
			</div>
		</div>
	}
}
