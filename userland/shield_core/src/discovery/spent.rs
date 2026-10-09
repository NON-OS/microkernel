//! Marking notes spent. The pool publishes nullifiers without their commitments, so the wallet
//! derives the nullifier of each note it holds and marks the note spent when it appears.

use crate::keys::{note_nullifier_wire, Account};
use crate::notes::NoteRecord;

/// The commitments among `notes` whose nullifier is in the published set `spent`.
pub fn spent_commitments(
    account: &Account,
    notes: &[NoteRecord],
    spent: &[[u8; 32]],
) -> Vec<[u64; 4]> {
    notes
        .iter()
        .filter(|note| spent.contains(&note_nullifier_wire(account, &note.cm, note.leaf_index)))
        .map(|note| note.cm)
        .collect()
}
