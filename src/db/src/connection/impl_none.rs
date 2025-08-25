use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use base64ct::Encoding as _;
use rand::{RngCore as _, rng};
use sea_orm::{
	ActiveValue::{Set, Unchanged},
	DerivePartialModel, FromQueryResult, QuerySelect as _, TransactionTrait as _,
	prelude::*,
};
// PERF: Consider usage of SecretBox, since doesn't provide significant end-to-end protection, but does prevent logging secrets.
use secrecy::{ExposeSecret as _, ExposeSecretMut as _, SecretBox, SecretString};
use time::{Duration, OffsetDateTime};
use tracing_unwrap::OptionExt as _;

use crate::{
	StorageError,
	connection::{ApplicantState, Connection, InterviewerState, LoggedIn, NoneState, RecruitmentManagerState},
	entities::{sea_orm_active_enums::UserType, session, user},
	secret::{Blake3Hash, PasswordHash},
};

/// Credentials generated for a session to be sent to the client
#[derive(Debug)]
#[non_exhaustive]
pub struct SessionCredentials {
	/// Session id
	pub id: String,
	/// Session secret token
	pub token: SecretString,
	/// Session expiration time
	pub expiration_time: OffsetDateTime,
}

impl Connection<NoneState> {
	/// Create a new applicant user account and return his ID.
	///
	/// # Errors
	/// - Failing to hash the password.
	/// - Failing to communicate with the database.
	#[tracing::instrument(skip(self), level = tracing::Level::DEBUG)]
	pub async fn create_applicant_user(&self, user_password: SecretString) -> Result<String, StorageError> {
		let user_id = Uuid::new_v4();

		let user = user::ActiveModel {
			id: Set(user_id),
			password: Set(PasswordHash::generate(user_password).await?),
			r#type: Set(UserType::Applicant),
			last_login_at: Set(TimeDateTimeWithTimeZone::now_utc()),
			to_change_password: Set(false),
		};

		// TODO: Generalize this log message for every user creation regardless of type.
		tracing::info!(user.id = ?user_id, user.type = ?UserType::Applicant, "creating new user");

		let user = user.insert(&self.connection);

		Ok(user.await?.id.to_string())
	}

	// TODO: Support 2FA and passkeys?
	/// Authenticate a user account and create a new session for him.
	///
	/// # Errors
	/// - Failing to parse the `user_id` as `Uuid`.
	/// - Failing to communicate with the database.
	#[tracing::instrument(skip(self), level = tracing::Level::DEBUG)]
	pub async fn create_session(
		&self,
		user_id: &str,
		user_password: SecretString,
	) -> Result<Option<SessionCredentials>, StorageError> {
		let user_id = Uuid::parse_str(user_id)?;

		#[derive(DerivePartialModel, FromQueryResult)]
		#[sea_orm(entity = "user::Entity")]
		struct PartialUser {
			id: Uuid,
			password: PasswordHash,
		}
		let user = user::Entity::find_by_id(user_id)
			.into_partial_model::<PartialUser>()
			.one(&self.connection);

		if let Some(user) = user.await?
			&& user.password.validate(user_password).await
		{
			// New session's id
			let id = Uuid::now_v7();

			// New session's token
			let mut token = SecretBox::new(Box::new([0_u8; 32]));
			rng().fill_bytes(token.expose_secret_mut());

			// For client storage
			let token_b64 = SecretString::from(base64ct::Base64Unpadded::encode_string(
				token.expose_secret().as_slice(),
			));
			drop(token);
			// For database storage (hashing the base64 bytes to avoid decoding in `load_session`)
			let secret_hash = Blake3Hash::generate(&token_b64);

			// TODO: Handle session expiration with a more advanced approach (maybe dynamic expiration to an extent?).
			// New session's expiration
			let expiration_time = OffsetDateTime::now_utc()
				.checked_add(Duration::hours(3))
				.unwrap_or_log();

			// Session creation database transaction
			let session_creation_transaction = self.connection.begin().await?;

			tracing::info!(session.id = ?id, session.expiration_time = ?expiration_time, "creating new session");

			#[expect(unused_results, reason = "No need")]
			session::ActiveModel {
				id: Set(id),
				secret: Set(secret_hash.await),
				user_id: Set(user.id),
				expires_at: Set(expiration_time),
			}
			.insert(&session_creation_transaction)
			.await?;

			// For the user
			let updated_last_login_time = OffsetDateTime::now_utc();

			tracing::debug!(?user.id, user.updated_last_login_time = ?updated_last_login_time, "updating user's last login time");

			#[expect(unused_results, reason = "No need")]
			user::ActiveModel {
				id: Unchanged(user.id),
				last_login_at: Set(updated_last_login_time),
				..Default::default()
			}
			.update(&session_creation_transaction)
			.await?;

			session_creation_transaction.commit().await?;

			Ok(Some(SessionCredentials {
				id: STANDARD_NO_PAD.encode(id.as_bytes()),
				token: token_b64,
				expiration_time,
			}))
		} else {
			tracing::warn!("faild to create new session, incorrect user id or password");

			Ok(None)
		}
	}

	/// Authenticate user session and change the database connection to `LoggedIn` state.
	///
	/// # Errors
	/// - Failing to decode and parse the session `id` as `Uuid`.
	/// - Failing to link a valid session with an existing user.
	/// - Failing to communicate with the database.
	#[tracing::instrument(skip(self), level = tracing::Level::DEBUG)]
	pub async fn load_session(self, id: &str, secret: SecretString) -> Result<LoggedIn, StorageError> {
		let id = Uuid::from_slice(&STANDARD_NO_PAD.decode(id)?)?;

		// Session id in the span is base64 encoded, so we are logging it here again
		tracing::debug!(session.id = ?id, " validating and loading session");

		#[derive(DerivePartialModel, FromQueryResult)]
		#[sea_orm(entity = "session::Entity")]
		struct PartialSession {
			secret: Blake3Hash,
			expires_at: OffsetDateTime,
		}
		#[derive(DerivePartialModel, FromQueryResult)]
		#[sea_orm(entity = "user::Entity")]
		struct PartialUser {
			id: Uuid,
			r#type: UserType,
		}
		let session = session::Entity::find_by_id(id)
			.find_also_related(user::Entity)
			// TODO: Aliases are specified manually due to a current limitation in sea-orm.
			// .into_partial_model::<PartialSession, PartialUser>()
			.select_only()
			.column_as(session::Column::Id, "A_id")
			.column_as(session::Column::Secret, "A_secret")
			.column_as(session::Column::ExpiresAt, "A_expires_at")
			.column_as(user::Column::Id, "B_id")
			.column_as(user::Column::Type, "B_type")
			.into_model::<PartialSession, PartialUser>()
			.one(&self.connection);

		match session.await? {
			Some((session, Some(user))) => {
				// Check for expiration first, since it is cheaper and more likely to fail
				if OffsetDateTime::now_utc() < session.expires_at {
					if session.secret.validate(secret).await {
						tracing::info!(?user.id, user.type = ?user.r#type, "successfully validated session");

						Ok(match user.r#type {
							UserType::Applicant => LoggedIn::Applicant(Connection {
								connection: self.connection,
								state: ApplicantState {},
							}),
							UserType::RecruitmentManager => LoggedIn::RecruitmentManager(Connection {
								connection: self.connection,
								state: RecruitmentManagerState {},
							}),
							UserType::Interviewer => LoggedIn::Interviewer(Connection {
								connection: self.connection,
								state: InterviewerState {},
							}),
						})
					} else {
						tracing::warn!(?user.id, user.type = ?user.r#type, "failed to load session, incorrect session secret");

						Ok(LoggedIn::None(self))
					}
				} else {
					tracing::warn!(?user.id, user.type = ?user.r#type, "failed to load session, expired session");

					Ok(LoggedIn::None(self))
				}
			}
			Some((_, None)) => Err(DbErr::RecordNotFound(String::from(
				"Valid session record must be associated with an existing user record",
			))
			.into()),
			None => {
				tracing::warn!("failed to load session, invalid session id");

				Ok(LoggedIn::None(self))
			}
		}
	}
}
