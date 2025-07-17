mod m20250714_015113_create_users_table;
mod m20250714_220820_create_sessions_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20250714_015113_create_users_table::Migration),
			Box::new(m20250714_220820_create_sessions_table::Migration),
		]
	}
}
