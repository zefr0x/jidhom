//! Database migrations
mod m20250714_015113_create_users_table;
mod m20250714_220820_create_sessions_table;
mod m20250813_171740_add_user_to_password_change_state;

pub use sea_orm_migration::prelude::*;

/// Database's `Migrator`
#[derive(Debug)]
#[non_exhaustive]
pub struct Migrator;

impl Migrator {
	/// Create a new database `Migrator`
	#[must_use]
	pub const fn new() -> Self {
		Self {}
	}
}

impl Default for Migrator {
	fn default() -> Self {
		Self::new()
	}
}

impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20250714_015113_create_users_table::Migration),
			Box::new(m20250714_220820_create_sessions_table::Migration),
			Box::new(m20250813_171740_add_user_to_password_change_state::Migration),
		]
	}
}
