//! Pages library for the web user interface
#![expect(clippy::exhaustive_structs, reason = "Leptos component macro problem")]
#![expect(clippy::must_use_candidate, reason = "Leptos component macro problem")]
#![expect(clippy::same_name_method, reason = "Leptos component macro problem")]

mod home;
mod login;
mod register;

pub use home::*;
pub use login::*;
pub use register::*;
