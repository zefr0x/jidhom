//! General utilities use by components

use leptos::prelude::*;
use leptos_use::{use_cookie, use_preferred_dark};

#[derive(Clone, Copy, Debug)]
struct IsActiveSession {
	getter: Signal<Option<bool>>,
	setter: WriteSignal<Option<bool>>,
}

#[derive(Clone, Copy, Debug)]
struct IsDarkTheme {
	getter: Signal<Option<bool>>,
	setter: WriteSignal<Option<bool>>,
}

/// Collection of cookies signals that have a dynamic effect in the client
#[derive(Clone, Copy, Debug)]
pub struct ClientCookies {
	is_active_session: IsActiveSession,
	is_dark_theme: IsDarkTheme,
}

impl IsActiveSession {
	fn new() -> Self {
		let (getter, setter) = use_cookie::<bool, codee::string::FromToStringCodec>("is_active_session");

		Self { getter, setter }
	}
}

impl IsDarkTheme {
	fn new() -> Self {
		// To figure out client's preference
		let is_browser_dark_preferred = use_preferred_dark();

		let (getter, setter) = use_cookie::<bool, codee::string::FromToStringCodec>("is_dark_theme");

		// HACK: Since `Sec-CH-Prefers-Color-Scheme` header is still experimental, we are using cookie.
		// For the first load, we don't have knowledge about the client's preference in SSR
		#[expect(unused_results, reason = "No need")]
		Effect::new(move || {
			setter(Some(is_browser_dark_preferred.get()));
		});

		Self { getter, setter }
	}
}

impl ClientCookies {
	/// Initialize an object with all signals.
	pub fn new() -> Self {
		Self {
			is_active_session: IsActiveSession::new(),
			is_dark_theme: IsDarkTheme::new(),
		}
	}

	// TODO: Use the `fn_traits` nightly feature to implement getters and setters.

	/// Get `is_active_session` cookie value
	pub fn is_active_session(self) -> bool {
		self.is_active_session.getter.get().is_some_and(|x| x)
	}

	/// Set `is_active_session` cookie value
	pub fn set_is_active_session(self, value: bool) {
		self.is_active_session.setter.set(Some(value));
	}

	/// Get `is_dark_theme` cookie value
	pub fn is_dark_theme(self) -> bool {
		// Default to dark theme, since it is less painful for light theme users in the first load
		self.is_dark_theme.getter.get().unwrap_or(true)
	}

	/// Set `is_dark_theme` cookie value
	pub fn set_is_dark_theme(self, value: Option<bool>) {
		self.is_dark_theme.setter.set(value);
	}
}

impl Default for ClientCookies {
	fn default() -> Self {
		Self::new()
	}
}
