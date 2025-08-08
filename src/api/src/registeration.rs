//! User creation interfaces.
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use secrecy::SecretString;

/// Create new applicant user (accessible for the public)
#[cfg_attr(
	feature = "ssr",
	tracing::instrument(ret, skip(user_password), fields(user_password = "[REDACTED]"), level = tracing::Level::TRACE)
)]
#[server]
pub async fn create_new_applicant_user(user_password: String) -> Result<String, ServerFnError> {
	// VULN: No password length limit (should be very large to mathematically cover long low entropy passphrase in 2^256).
	let user_password = SecretString::from(user_password);

	let con = crate::utils::get_database_connection().await?;

	con.create_applicant_user(user_password).await.map_err(|storage_error| {
		tracing::error!(?storage_error);

		ServerFnError::ServerError(
			"Failed to create applicant user. View/enable debug logs for more details.".to_owned(),
		)
	})
}
