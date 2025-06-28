use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

#[component]
pub fn Provider(children: Children) -> impl IntoView {
	leptos_fluent! {
		children: children(),
		locales: "./locales",
		default_language: "ar",
		set_language_to_cookie: true,
		initial_language_from_url_param: true,
		initial_language_from_cookie: true,
		initial_language_from_navigator: true,
		initial_language_from_accept_language_header: true,
		cookie_name: "lang",
		cookie_attrs: "SameSite=Strict; Secure; Path=/",
		url_param: "lang",
		sync_html_tag_lang: true,
		sync_html_tag_dir: true,
		check_translations: true,
	}
}
