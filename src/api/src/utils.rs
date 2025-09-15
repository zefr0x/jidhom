#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

/// This should be used inside leptos server functions.
#[cfg(feature = "ssr")]
pub async fn get_database_connection() -> Result<db::Connection<db::NoneState>, ServerFnErrorErr> {
	leptos_actix::extract::<web::ThinData<db::Connection<db::NoneState>>>()
		.await
		.map(|data| data.0)
}

/// This should be used inside leptos server functions that require authentication.
#[cfg(feature = "ssr")]
pub async fn load_session(con: db::Connection<db::NoneState>) -> Result<db::LoggedIn, db::StorageError> {
	use leptos_actix::Request;
	use secrecy::SecretString;

	let request = expect_context::<Request>();

	let session_id = request.cookie("id");
	let session_secret = request.cookie("secret");

	if let (Some(session_id), Some(session_secret)) = (session_id, session_secret) {
		con.load_session(session_id.value(), SecretString::from(session_secret.value()))
			.await
	} else {
		tracing::warn!("failed to load session, credentials are partially or fully missing");

		Ok(db::LoggedIn::None(con))
	}
}
