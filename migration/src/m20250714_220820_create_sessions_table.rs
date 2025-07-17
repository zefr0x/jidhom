use sea_orm_migration::{prelude::*, schema::*};

use super::m20250714_015113_create_users_table::User;

#[derive(DeriveIden)]
enum Session {
	Table,
	Id,
	Secret,
	UserId,
	// TODO: Create a job to delete expired sessions.
	ExpiresAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Session::Table)
					.if_not_exists()
					.col(pk_uuid(Session::Id))
					.col(binary_len(Session::Secret, 32))
					.col(uuid(Session::UserId))
					.col(timestamp_with_time_zone(Session::ExpiresAt))
					.foreign_key(
						ForeignKey::create()
							.from(Session::Table, Session::UserId)
							.to(User::Table, User::Id)
							.on_delete(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.drop_table(Table::drop().table(Session::Table).to_owned()).await
	}
}
