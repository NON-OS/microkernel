//! One row of the store, and the two directions it travels. An unknown kind is refused, since
//! reading around it could make a spent note spendable again.

// Crate visible so the fuzz surface can hand it a row payload directly.
pub(crate) mod decode;
mod encode;
mod status;

use crate::notes::{NoteRecord, NoteStatus};

pub(super) const FOUND: u8 = 1;
pub(super) const STATUS: u8 = 2;
pub(super) const CURSOR: u8 = 3;
/// A deposit sent and not yet seen stored: the opening, which exists nowhere
/// else until the pool stores its leaf. A new kind, so older stores still read.
pub(super) const DEPOSIT: u8 = 4;
/// One thing the wallet did with the pool (`store::activity`). A new kind, so older stores still
/// read; an older build refuses a store holding one, as it must refuse any kind it does not know.
pub(super) const ACTIVITY: u8 = 5;

/// What one row says.
pub enum Row {
    /// A note this wallet opened, with the position the pool gave it.
    Found(NoteRecord),
    /// A note's status moved.
    Status { cm: [u64; 4], status: NoteStatus },
    /// The scan reached this output index.
    Cursor(u64),
    /// A deposit this wallet built and sent, not yet seen in the pool. Its
    /// opening is the only copy of the note's secret until the leaf is stored.
    Deposit(crate::notes::NotePlaintext),
    /// One entry of the history.
    Activity(crate::store::activity::Activity),
}

pub use decode::decode;
