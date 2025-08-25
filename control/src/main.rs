//! Admin control CLI tool.
#![allow(clippy::print_stdout, reason = "CLI tool")]

mod cli;

use anyhow::Context as _;
use secrecy::ExposeSecret as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Load `.env` file in debug builds
	#[cfg(debug_assertions)]
	#[expect(unused_must_use, reason = "No need")]
	dotenvy::dotenv();

	// Initialize tracing subscriber
	tracing_subscriber::fmt()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.with_writer(std::io::stderr)
		.init();

	// Parse CLI arguments
	tracing::trace!("Parsing CLI arguments");
	let matches = cli::build().get_matches();

	// Load environment variables
	let database_url = std::env::var("DATABASE_URL").context("you must have `DATABASE_URL` set")?;

	// Create a database connections pool
	let database_connection = db::Connection::<db::AdminState>::new_for_admin(&database_url).await?;

	// Match command and arguments to do actions

	tracing::debug!(?matches, "Matching CLI arguments");

	match matches.subcommand() {
		Some(("user", matches)) => match matches.subcommand() {
			Some(("new-recruitment-manager", _)) => {
				let new_user_credentials = database_connection.create_recruitment_manager_user().await?;

				println!(
					"### Created New Recruitment Manager User Successfully ###\
					\n\nUser ID: {}\
					\nUser Password: {}",
					new_user_credentials.id,
					new_user_credentials.password.expose_secret()
				);
			}
			Some(("reset-password", matches)) => {
				let user_id = matches.get_one::<String>("user_id").unwrap();

				let new_password = database_connection.reset_user_password(user_id);

				println!(
					"### Resetted User Password Successfully ###\
					\n\nUser ID: {}\
					\nNew Password: {}",
					user_id,
					new_password.await?.expose_secret()
				);
			}
			_ => unreachable!(),
		},
		_ => unreachable!(),
	}

	Ok(())
}
