//! Leptos web components library to be used to construct pages in the `pages` module
#![expect(clippy::exhaustive_structs, reason = "Leptos component macro problem")]
#![expect(clippy::must_use_candidate, reason = "Leptos component macro problem")]
#![expect(clippy::same_name_method, reason = "Leptos component macro problem")]

mod nav;

pub use nav::*;
