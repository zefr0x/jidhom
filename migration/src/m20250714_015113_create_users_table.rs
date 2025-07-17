use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum User {
	Table,
	Id,
	Type,
	Password,
	// TODO: Create a job to delete old users without recent interaction.
	LastLoginAt,
}

#[derive(DeriveIden)]
struct UserType;

#[derive(DeriveIden, EnumIter)]
enum UserTypeVariants {
	Applicant,
	RecruitmentManager,
	Interviewer,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_type(
				extension::postgres::Type::create()
					.as_enum(UserType)
					.values(UserTypeVariants::iter())
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(User::Table)
					.if_not_exists()
					.col(pk_uuid(User::Id))
					.col(enumeration(User::Type, UserType, UserTypeVariants::iter()))
					// NOTE: This is a hash string formatted with the PHC spec.
					.col(text(User::Password))
					.col(timestamp_with_time_zone(User::LastLoginAt))
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;

		manager
			.drop_type(extension::postgres::Type::drop().name(UserType).to_owned())
			.await
	}
}
