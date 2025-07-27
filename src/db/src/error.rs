#[expect(missing_docs, reason = "Variants' names and error messages are descriptive")]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum StorageError {
	#[error("Database error occurred: {0}")]
	DatabaseError(#[from] sea_orm::DbErr),
	#[error("Error occurred while hashing the password")]
	PasswordHashError(#[from] password_hash::Error),
	#[error("Failed to parse UUID: {0}")]
	UuidError(#[from] uuid::Error),
	#[error("Base64 decoding failed: {0}")]
	Base64DecodeError(#[from] base64::DecodeError),
}
