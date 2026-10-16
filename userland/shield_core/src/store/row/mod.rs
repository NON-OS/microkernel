//! One row of the store, and the two directions it travels. An unknown kind is refused, since
//! reading around it could make a spent note spendable again.

// Crate visible so the fuzz surface can hand it a row payload directly.
pub(crate) mod decode;
mod encode;
mod entry;
mod kind;
mod status;

use kind::{ACTIVITY, CURSOR, DEPOSIT, FOUND, STATUS};

pub use decode::decode;
pub use entry::Row;
