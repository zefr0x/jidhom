use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_use::{UseClipboardReturn, use_clipboard};
use phosphor_leptos::{CLIPBOARD_TEXT, Icon, IconWeight, X};

/// Applicant user registration widget
#[component]
pub fn ApplicantRegisterWidget() -> impl IntoView {
	let create_new_applicant_user = ServerAction::<api::registeration::CreateNewApplicantUser>::new();
	let user_id = create_new_applicant_user.value();
	let registeration_pending = create_new_applicant_user.pending();

	let user_id_unwraped = move || {
		if let Some(Ok(user_id)) = user_id.get() {
			user_id
		} else {
			String::new()
		}
	};

	let UseClipboardReturn {
		is_supported: is_clipboard_supported,
		copy,
		..
	} = use_clipboard();

	view! {
		<div class="card bg-base-100 w-full max-w-sm shrink-0 shadow-2xl">
			<div class="card-body">
				<ActionForm action=create_new_applicant_user>
					<fieldset class="fieldset">
						<legend class="fieldset-legend">{move_tr!("register-as-applicant")}</legend>

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

						<button type="submit" class="btn btn-neutral mt-4" class:btn-disabled=move || registeration_pending.get()>
							<Show when=move || registeration_pending.get()>
								<span class="loading loading-spinner"></span>
							</Show>
							{move_tr!("create-account")}
						</button>
					</fieldset>
				</ActionForm>
			</div>
		</div>

		<dialog class="modal modal-bottom sm:modal-middle" open=move || user_id.get().is_some_and(|x| x.is_ok())>
			<div class="modal-box">
				<form method="dialog">
					<button
						class="btn btn-sm btn-circle btn-ghost absolute end-2 top-2"
						on:click=move |_| {create_new_applicant_user.clear();}
					>
						<i><Icon weight=IconWeight::Bold icon=X /></i>
					</button>
				</form>

				<h3 class="text-lg font-bold">{move_tr!("registered-successfully")}</h3>

				<p class="py-4">
					<label class="input w-full mb-5">
						{move_tr!("user-identifier")}

						// FIX: Part of the ID might not be showed.
						<input
							type="text"
							value={user_id_unwraped}
							class="input"
							disabled
						/>

						<Show when=move || is_clipboard_supported()>
							<button
								class="btn btn-square btn-sm btn-primary btn-outline"
								title={move_tr!("copy-id")}
								on:click={
									let copy = copy.clone();
									move |_| copy(&user_id_unwraped())
								}
							>
								<i><Icon weight=IconWeight::Fill size="24px" icon=CLIPBOARD_TEXT /></i>
							</button>
						</Show>
					</label>

					{move_tr!("user-id-is-important")}
				</p>
			</div>
		</dialog>

	}
}
