// NONOS Operating System (AGPL-3.0-or-later)
//! Only the request builder half of the directory fetch.
//!
//! The response half inflates zlib through `nonos_inflate`, a dependency this crate
//! does not carry, and nothing here needs it: what is checked is how a request is
//! addressed, not how an answer is unpacked.

#[path = "../../../../capsule_net_anon/src/directory/fetch/request.rs"]
pub mod request;

pub use request::micro_path;
