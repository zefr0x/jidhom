#[cfg(feature = "admin")]
mod impl_admin;
mod impl_applicant;
mod impl_interviewer;
mod impl_loggedin;
mod impl_none;
mod impl_recruitment_manager;

use sea_orm::{Database, DatabaseConnection};

use crate::StorageError;

/// General connection state trait
pub trait State {}
/// `LoggedIn` user connection state trait
pub trait LoggedInState {}

/// The empty connection state (should be the default to be shared with the whole application)
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct NoneState;
impl State for NoneState {}

/// Admin connection state (should be accessible only from the admin control)
#[derive(Debug)]
#[non_exhaustive]
#[cfg(feature = "admin")]
pub struct AdminState;
#[cfg(feature = "admin")]
impl State for AdminState {}

// LoggedIn states

/// Applicant user connection state
#[derive(Debug)]
#[non_exhaustive]
pub struct ApplicantState;
impl State for ApplicantState {}
impl LoggedInState for ApplicantState {}

/// Recruitment manager user connection state
#[derive(Debug)]
#[non_exhaustive]
pub struct RecruitmentManagerState;
impl State for RecruitmentManagerState {}
impl LoggedInState for RecruitmentManagerState {}

/// Interviewer user connection state
#[derive(Debug)]
#[non_exhaustive]
pub struct InterviewerState;
impl State for InterviewerState {}
impl LoggedInState for InterviewerState {}

/// `LoggedIn` user states enum
#[expect(missing_docs, reason = "Variants' names are descriptive")]
#[derive(Debug)]
#[non_exhaustive]
pub enum LoggedIn {
	None(Connection<NoneState>),
	Applicant(Connection<ApplicantState>),
	RecruitmentManager(Connection<RecruitmentManagerState>),
	Interviewer(Connection<InterviewerState>),
}

/// The stateful database connection
#[derive(Clone, Debug)]
pub struct Connection<S> {
	connection: DatabaseConnection,
	#[expect(dead_code, reason = "Not used yet")]
	state: S,
}

// TODO: Figure a way to make this testable with the Mock interface.
/// General methods for all states
impl<S> Connection<S>
where
	S: State,
{
	/// Create a new database connection pool to be accessed by the whole application.
	///
	/// # Errors
	/// - Failing to initialize a communication with the database.
	pub async fn new(database_url: &str) -> Result<Connection<NoneState>, StorageError> {
		tracing::info!(
			target_database_url = database_url,
			"creating `NoneState` database connection pool"
		);

		let connection = Database::connect(database_url).await?;

		Ok(Connection {
			connection,
			state: NoneState {},
		})
	}

	/// Create a new database connection pool for the admin.
	///
	/// # Errors
	/// - Failing to initialize a communication with the database.
	#[cfg(feature = "admin")]
	pub async fn new_for_admin(database_url: &str) -> Result<Connection<AdminState>, StorageError> {
		tracing::info!(
			target_database_url = database_url,
			"creating `AdminState` database connection pool"
		);

		let connection = Database::connect(database_url).await?;

		Ok(Connection {
			connection,
			state: AdminState {},
		})
	}
}
