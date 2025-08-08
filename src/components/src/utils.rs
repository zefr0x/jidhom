//! General utilities use by components

use leptos::prelude::*;
use leptos_use::use_cookie;

/// Collection of cookies signals that have a dynamic effect in the client
#[derive(Clone, Copy, Debug)]
pub struct ClientCookies {
	is_active_session: IsActiveSession,
}

#[derive(Clone, Copy, Debug)]
struct IsActiveSession {
	inner: (Signal<Option<bool>>, WriteSignal<Option<bool>>),
}

impl ClientCookies {
	/// Initialize an object with all signals.
	pub fn new() -> Self {
		Self {
			is_active_session: IsActiveSession {
				inner: use_cookie::<bool, codee::string::FromToStringCodec>("is_active_session"),
			},
		}
	}

	// TODO: Use the `fn_traits` nightly feature to implement getters and setters.

	/// Get `is_active_session` cookie value
	pub fn is_active_session(self) -> bool {
		self.is_active_session.inner.0().is_some_and(|x| x)
	}

	/// Set `is_active_session` cookie value
	pub fn set_is_active_session(self, value: bool) {
		self.is_active_session.inner.1(Some(value));
	}
}

impl Default for ClientCookies {
	fn default() -> Self {
		Self::new()
	}
}
