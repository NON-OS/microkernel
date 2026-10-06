//! A note this wallet can spend, as the store keeps it.
//! The nullifier hashes the position, so a wrong position leaves the note unable to retire.

use super::plaintext::NotePlaintext;

/// Where a note stands. Pending spans proof sent to batch settled: not spendable, not gone.
/// Collapsing it into unspent would build a second transfer over the same note.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoteStatus {
    Unspent,
    Pending,
    Spent,
}

/// A spendable note: its opened plaintext, where the pool put it, and its status.
pub struct NoteRecord {
    pub plain: NotePlaintext,
    /// The leaf position the pool authenticated, which the nullifier hashes.
    pub leaf_index: u64,
    /// The commitment, as the four field words the pool holds.
    pub cm: [u64; 4],
    pub status: NoteStatus,
    /// The output index this note was found at, so a rescan can resume.
    pub found_at: u64,
}

impl core::fmt::Debug for NoteRecord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NoteRecord")
            .field("leaf_index", &self.leaf_index)
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}
