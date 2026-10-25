//! What one row says, whichever kind it is.

use crate::notes::{NoteRecord, NoteStatus};

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
