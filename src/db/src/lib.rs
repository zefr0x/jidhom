mod entities;

use sea_orm::{Database, DatabaseConnection, DbErr};

#[expect(dead_code)]
#[derive(Clone)]
pub struct Connection {
	connection: DatabaseConnection,
}

impl Connection {
	pub async fn new(database_url: &str) -> Result<Self, DbErr> {
		let connection = Database::connect(database_url).await?;

		log::info!("Connected to {database_url}");

		Ok(Self { connection })
	}
}
