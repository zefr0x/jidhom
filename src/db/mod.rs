mod entities;

use leptos::logging::log;
use sea_orm::{Database, DatabaseConnection, DbErr};

#[expect(dead_code)]
#[derive(Clone)]
pub struct Connection {
	connection: DatabaseConnection,
}

impl Connection {
	pub async fn new(database_url: &str) -> Result<Self, DbErr> {
		let connection = Database::connect(database_url).await?;

		log!("Created connection pool for {database_url}");

		Ok(Self { connection })
	}
}
