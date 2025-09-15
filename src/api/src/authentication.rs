//! Session and authentication interfaces.
#[cfg(feature = "ssr")]
use actix_web::{
	cookie::{Cookie, SameSite},
	http::header::{self, HeaderValue},
};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use secrecy::{ExposeSecret as _, SecretString};

use crate::SessionRole;
#[cfg(feature = "ssr")]
use crate::utils::{get_database_connection, load_session};

/// Login/creating session for any user
#[cfg_attr(feature = "ssr", tracing::instrument(level = tracing::Level::TRACE))]
#[server]
pub async fn create_new_session(user_id: String, user_password: String) -> Result<bool, ServerFnError> {
	// VULN: No password length limit validation.
	let user_password = SecretString::from(user_password);

	let con = get_database_connection().await?;

	match con.create_session(&user_id, user_password).await {
		Ok(Some(session)) => {
			let response = expect_context::<ResponseOptions>();

			// Set session id cookie
			response.append_header(
				header::SET_COOKIE,
				HeaderValue::from_str(
					&Cookie::build("id", &session.id)
						.path("/")
						.expires(session.expiration_time)
						.secure(true)
						.http_only(true)
						.same_site(SameSite::Strict)
						.finish()
						.to_string(),
				)
				.unwrap(),
			);
			// Set session secret token cookie
			response.append_header(
				header::SET_COOKIE,
				HeaderValue::from_str(
					&Cookie::build("secret", session.token.expose_secret())
						.path("/")
						.expires(session.expiration_time)
						.secure(true)
						.http_only(true)
						.same_site(SameSite::Strict)
						.finish()
						.to_string(),
				)
				.unwrap(),
			);
			// Set logged in state cookie to true
			response.append_header(
				header::SET_COOKIE,
				HeaderValue::from_str(
					// This value will be parsed as a `bool`, either "true" or "false"
					&Cookie::build("is_active_session", "true")
						.path("/")
						.expires(session.expiration_time)
						.secure(true)
						.same_site(SameSite::Strict)
						.finish()
						.to_string(),
				)
				.unwrap(),
			);

			Ok(true)
		}
		Ok(None) => Ok(false),
		Err(storage_error) => {
			tracing::error!(?storage_error);

			Err(ServerFnError::ServerError(
				"Failed to create session. View/enable debug logs for more details.".to_owned(),
			))
		}
	}
}

#[cfg_attr(feature = "ssr", tracing::instrument(level = tracing::Level::TRACE))]
#[server]
pub async fn get_session_role() -> Result<SessionRole, ServerFnError> {
	match load_session(get_database_connection().await?).await {
		Ok(session) => Ok(match session {
			db::LoggedIn::Applicant(_) => SessionRole::Applicant,
			db::LoggedIn::Interviewer(_) => SessionRole::Interviewr,
			db::LoggedIn::RecruitmentManager(_) => SessionRole::RecruitmentManager,
			db::LoggedIn::None(_) | _ => SessionRole::Undetermined,
		}),
		Err(storage_error) => {
			tracing::error!(?storage_error);

			Err(ServerFnError::ServerError(
				"Failed to get session role. View/enable debug logs for more details.".to_owned(),
			))
		}
	}
}
