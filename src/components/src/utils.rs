//! General utilities use by components

use leptos::prelude::*;
use leptos_use::{use_cookie, use_preferred_dark};

/// Collection of cookies signals that have a dynamic effect in the client
#[derive(Clone, Copy, Debug)]
pub struct ClientCookies {
	is_active_session: IsActiveSession,
	is_dark_theme: IsDarkTheme,
}

#[derive(Clone, Copy, Debug)]
struct IsActiveSession {
	inner: (Signal<Option<bool>>, WriteSignal<Option<bool>>),
}

#[derive(Clone, Copy, Debug)]
struct IsDarkTheme {
	inner: (Signal<Option<bool>>, WriteSignal<Option<bool>>),
}

impl ClientCookies {
	/// Initialize an object with all signals.
	pub fn new() -> Self {
		// Dark/Light theme management

		// To figure out client's preference
		let is_browser_dark_preferred = use_preferred_dark();

		let (color_theme, set_color_theme) = use_cookie::<bool, codee::string::FromToStringCodec>("is_dark_theme");

		// HACK: Since `Sec-CH-Prefers-Color-Scheme` header is still experimental, we are using cookie.
		// For the first load, we don't have knowledge about the client's preference in SSR
		#[expect(unused_results, reason = "No need")]
		Effect::new(move || {
			set_color_theme(Some(is_browser_dark_preferred.get()));
		});

		// Construct client cookies struct
		Self {
			is_active_session: IsActiveSession {
				inner: use_cookie::<bool, codee::string::FromToStringCodec>("is_active_session"),
			},
			is_dark_theme: IsDarkTheme {
				inner: (color_theme, set_color_theme),
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

	/// Get `is_dark_theme` cookie value
	pub fn is_dark_theme(self) -> bool {
		// Default to dark theme, since it is less painful for light theme users in the first load
		self.is_dark_theme.inner.0().unwrap_or(true)
	}

	/// Set `is_dark_theme` cookie value
	pub fn set_is_dark_theme(self, value: Option<bool>) {
		self.is_dark_theme.inner.1(value);
	}
}

impl Default for ClientCookies {
	fn default() -> Self {
		Self::new()
	}
}
