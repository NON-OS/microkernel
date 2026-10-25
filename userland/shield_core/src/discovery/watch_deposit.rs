//! An own deposit confirmed: the escrowed value and the recomputed leaf must both match.

use super::{leaf_of, WatchReject};
use crate::notes::{NotePlaintext, NoteRecord, NoteStatus};

/// Confirm an own deposit: the escrowed value and the recomputed leaf must both match.
pub fn confirm_deposit(
    plain: &NotePlaintext,
    leaf_index: u64,
    emitted_leaf: &[u8; 32],
    emitted_value: u64,
) -> Result<NoteRecord, WatchReject> {
    if plain.value != emitted_value {
        return Err(WatchReject::Value);
    }
    let (cm, ours) = leaf_of(plain);
    if &ours != emitted_leaf {
        return Err(WatchReject::Leaf);
    }
    Ok(NoteRecord {
        plain: plain.clone(),
        leaf_index,
        cm,
        status: NoteStatus::Unspent,
        found_at: leaf_index,
    })
}
