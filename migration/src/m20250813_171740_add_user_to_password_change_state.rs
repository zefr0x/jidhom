use sea_orm_migration::{prelude::*, schema::boolean};

#[derive(DeriveIden)]
pub enum User {
	Table,
	ToChangePassword,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(User::Table)
					.add_column_if_not_exists(boolean(User::ToChangePassword))
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(User::Table)
					.drop_column(User::ToChangePassword)
					.to_owned(),
			)
			.await
	}
}
