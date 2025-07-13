#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

/// This should be used inside leptos server functions.
#[expect(dead_code)]
#[cfg(feature = "ssr")]
pub async fn get_database_connection() -> Result<web::ThinData<db::Connection>, ServerFnErrorErr> {
	leptos_actix::extract::<web::ThinData<db::Connection>>().await
}
