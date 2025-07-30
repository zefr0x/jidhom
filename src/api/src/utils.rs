#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

/// This should be used inside leptos server functions.
#[expect(dead_code, reason = "Not used yet")]
#[cfg(feature = "ssr")]
pub async fn get_database_connection() -> Result<db::Connection<db::NoneState>, ServerFnErrorErr> {
	leptos_actix::extract::<web::ThinData<db::Connection<db::NoneState>>>()
		.await
		.map(|data| data.0)
}
