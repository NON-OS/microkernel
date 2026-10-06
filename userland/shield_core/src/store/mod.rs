//! The note store: an append only log of rows sealed under a seed-derived key.
//! A phone seized unlocked yields rows nothing can read once the wallet is locked.

pub mod activity;
mod append;
// Crate visible so the fuzz surface can hand it bytes directly. Nothing
// outside this crate can see it either way.
pub(crate) mod frame;
mod key;
mod log;
pub(crate) mod pool;
mod query;
pub(crate) mod reader;
mod replay;
/// One row of the log, and the two directions it travels.
pub mod row;
mod state;

pub use log::NoteLog;
pub use row::Row;
pub use state::{Balance, StoreState};
