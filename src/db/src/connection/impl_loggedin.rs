use crate::connection::{Connection, LoggedInState};

// General methods for all logged in users
impl<S> Connection<S> where S: LoggedInState {}
