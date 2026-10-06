//! A note's status on the wire, one byte for three states. Any other byte is refused, because
//! defaulting it to unspent would revive a spent note after an upgrade.

use crate::error::StoreError;
use crate::notes::NoteStatus;

/// The byte a status is stored as.
pub fn to_byte(s: NoteStatus) -> u8 {
    match s {
        NoteStatus::Unspent => 0,
        NoteStatus::Pending => 1,
        NoteStatus::Spent => 2,
    }
}

/// The status a byte stands for, or `RowKind` for a byte this version does not know.
pub fn from_byte(b: u8) -> Result<NoteStatus, StoreError> {
    match b {
        0 => Ok(NoteStatus::Unspent),
        1 => Ok(NoteStatus::Pending),
        2 => Ok(NoteStatus::Spent),
        _ => Err(StoreError::RowKind),
    }
}
