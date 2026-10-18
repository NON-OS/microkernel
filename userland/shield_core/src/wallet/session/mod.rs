//! A live wallet: the accounts of an unlocked seed, each with its own store. Locking drops it.
//! Every call about keys, notes or the public account reaches the active account.

mod accounts;
pub(crate) mod book;
mod candidates;
mod install;
mod live;
pub(super) mod open;
mod pool;
mod public;
mod restore;
pub(crate) mod slot;

pub use live::Session;
