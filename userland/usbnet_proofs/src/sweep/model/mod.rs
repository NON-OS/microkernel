// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound reader owes, from the descriptors as built.

mod least;
mod pipes;
mod scope;

pub use least::least;
pub use pipes::interfaces;
pub use scope::{functional, notify, status};
