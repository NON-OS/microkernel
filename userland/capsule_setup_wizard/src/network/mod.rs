//! The network step: no network (the default), or a Wi-Fi network joined
//! with its passphrase and, on a mode that keeps state, remembered sealed.

mod join;
mod keep;
mod keys;
mod poll;
mod state;
mod typing;
mod wired;

pub use keep::keep;
pub use keys::on_key;
pub use poll::{poll, wait_ms};
pub use state::{NetState, NETS_MAX};
pub use wired::wired_present;
