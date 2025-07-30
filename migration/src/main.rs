//! CLI for migrating the database
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
	cli::run_cli(migration::Migrator::new()).await;
}
