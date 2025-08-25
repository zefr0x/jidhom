use secrecy::SecretString;
use time::OffsetDateTime;

/// Credentials generated for a new user.
#[derive(Debug)]
#[non_exhaustive]
pub struct UserCredentials {
	/// User id
	pub id: String,
	/// User temporary secret token
	pub password: SecretString,
}

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
