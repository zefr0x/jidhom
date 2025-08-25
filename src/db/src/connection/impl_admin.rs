use base64ct::Encoding as _;
use rand::{RngCore as _, rng};
use sea_orm::{
	ActiveValue::{Set, Unchanged},
	prelude::*,
};
use secrecy::{ExposeSecret as _, ExposeSecretMut as _, SecretBox, SecretString};

use crate::{
	StorageError, UserCredentials,
	connection::{AdminState, Connection},
	entities::{sea_orm_active_enums::UserType, user},
	secret::PasswordHash,
};

impl Connection<AdminState> {
	/// Create a new recruitment manager user account and return his credentials.
	///
	/// The generated password is temporary, so the user will be asked to change it.
	///
	/// # Errors
	/// - Failing to hash the generated password.
	/// - Failing to communicate with the database.
	#[tracing::instrument(skip(self), level = tracing::Level::DEBUG)]
	pub async fn create_recruitment_manager_user(&self) -> Result<UserCredentials, StorageError> {
		let user_id = Uuid::new_v4();

		let mut token = SecretBox::new(Box::new([0_u8; 32]));
		rng().fill_bytes(token.expose_secret_mut());

		// For client storage
		let token_b64 = SecretString::from(base64ct::Base64Unpadded::encode_string(
			token.expose_secret().as_slice(),
		));
		drop(token);

		let user = user::ActiveModel {
			id: Set(user_id),
			password: Set(PasswordHash::generate(token_b64.clone()).await?),
			r#type: Set(UserType::RecruitmentManager),
			last_login_at: Set(TimeDateTimeWithTimeZone::now_utc()),
			to_change_password: Set(true),
		};

		tracing::info!(user.id = ?user_id, user.type = ?UserType::RecruitmentManager, "creating new user");

		let user = user.insert(&self.connection);

		Ok(UserCredentials {
			id: user.await?.id.to_string(),
			password: token_b64,
		})
	}

	/// Reset a user's password and return a new temporary password.
	///
	/// The generated password is temporary, so the user will be asked to change it.
	///
	/// # Errors
	/// - Failing to parse user's id.
	/// - Failing to hash the generated password.
	/// - Failing to communicate with the database.
	/// - No matching user id in the database.
	#[tracing::instrument(skip(self), level = tracing::Level::DEBUG)]
	pub async fn reset_user_password(&self, user_id: &str) -> Result<SecretString, StorageError> {
		let user_id = Uuid::parse_str(user_id)?;

		let mut token = SecretBox::new(Box::new([0_u8; 32]));
		rng().fill_bytes(token.expose_secret_mut());

		let token_b64 = SecretString::from(base64ct::Base64Unpadded::encode_string(
			token.expose_secret().as_slice(),
		));
		drop(token);

		tracing::info!(user.id = ?user_id, "Resetting user password");

		#[expect(unused_results, reason = "No need")]
		user::ActiveModel {
			id: Unchanged(user_id),
			password: Set(PasswordHash::generate(token_b64.clone()).await?),
			to_change_password: Set(true),
			..Default::default()
		}
		.update(&self.connection)
		.await?;

		Ok(token_b64)
	}
}
