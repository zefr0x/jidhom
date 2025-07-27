use base64ct::{Base64Unpadded, Encoding as _};
use password_hash::PasswordHasher as _;
use rand::{Rng as _, rng};
use secrecy::{ExposeSecret as _, ExposeSecretMut as _, SecretString};

use crate::utils::spawn_cpu_blocking;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash {
	inner: password_hash::PasswordHashString,
}

impl PasswordHash {
	pub async fn generate(mut password: SecretString) -> Result<Self, password_hash::Error> {
		spawn_cpu_blocking(move || {
			let salt: [u8; argon2::RECOMMENDED_SALT_LEN] = rng().random();

			let salt = Base64Unpadded::encode_string(&salt);

			Ok(Self {
				inner: argon2::Argon2::default()
					.hash_password(
						password.expose_secret_mut().as_bytes(),
						password_hash::Salt::from_b64(&salt).unwrap(),
					)?
					.into(),
			})
		})
		.await
		.unwrap()
	}

	pub async fn validate(self, mut password: SecretString) -> bool {
		spawn_cpu_blocking(move || {
			let algorithms: &[&dyn password_hash::PasswordVerifier] = &[&argon2::Argon2::default()];

			self.inner
				.password_hash()
				.verify_password(algorithms, password.expose_secret_mut().as_bytes())
				.is_ok()
		})
		.await
		.unwrap()
	}
}

impl From<PasswordHash> for sea_orm::Value {
	fn from(value: PasswordHash) -> Self {
		Self::String(Some(Box::new(value.inner.to_string())))
	}
}

impl sea_orm::sea_query::ValueType for PasswordHash {
	fn type_name() -> String {
		String::from("PasswordHash")
	}

	fn column_type() -> sea_orm::ColumnType {
		sea_orm::ColumnType::Text
	}

	fn array_type() -> sea_orm::sea_query::ArrayType {
		sea_orm::sea_query::ArrayType::String
	}

	fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
		if let sea_orm::Value::String(Some(s)) = v {
			let hash = password_hash::PasswordHashString::new(&s);

			hash.map_or(Err(sea_orm::sea_query::ValueTypeErr), |hash| Ok(Self { inner: hash }))
		} else {
			Err(sea_orm::sea_query::ValueTypeErr)
		}
	}
}

impl sea_orm::TryGetable for PasswordHash {
	fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
		let v: String = String::try_get_by(res, index)?;

		Ok(Self {
			inner: password_hash::PasswordHashString::new(&v)
				.map_err(|err| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(format!("{err:?}"))))?,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blake3Hash {
	inner: blake3::Hash,
}

impl Blake3Hash {
	// Using reference since tokens should be returned to the client and not dropped after hashing
	pub async fn generate(token: &SecretString) -> Self {
		spawn_cpu_blocking({
			// PERF: Any way to avoid clone?
			let token = token.to_owned();

			move || Self {
				inner: blake3::hash(token.expose_secret().as_bytes()),
			}
		})
		.await
		.unwrap()
	}

	pub async fn validate(self, mut token: SecretString) -> bool {
		spawn_cpu_blocking(move || self.inner == blake3::hash(token.expose_secret_mut().as_bytes()))
			.await
			.unwrap()
	}
}

impl From<Blake3Hash> for sea_orm::Value {
	fn from(value: Blake3Hash) -> Self {
		Self::Bytes(Some(Box::new(value.inner.as_bytes().to_vec())))
	}
}

impl sea_orm::sea_query::ValueType for Blake3Hash {
	fn type_name() -> String {
		String::from("Blake3Hash")
	}

	fn column_type() -> sea_orm::ColumnType {
		sea_orm::ColumnType::Binary(32)
	}

	fn array_type() -> sea_orm::sea_query::ArrayType {
		sea_orm::sea_query::ArrayType::Bytes
	}

	fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
		if let sea_orm::Value::Bytes(Some(bytes)) = v {
			let hash = blake3::Hash::from_slice(&bytes);

			hash.map_or(Err(sea_orm::sea_query::ValueTypeErr), |hash| Ok(Self { inner: hash }))
		} else {
			Err(sea_orm::sea_query::ValueTypeErr)
		}
	}
}

impl sea_orm::TryGetable for Blake3Hash {
	fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, index: I) -> Result<Self, sea_orm::TryGetError> {
		let v: Vec<u8> = Vec::try_get_by(res, index)?;

		Ok(Self {
			inner: blake3::Hash::from_slice(v.as_slice())
				.map_err(|err| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(format!("{err:?}"))))?,
		})
	}
}
