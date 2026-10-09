//! Which held notes are on the active pool. A note from another pool, the launch pool before v2,
//! keeps its row, since that pool can still pay it out, but it is neither counted nor spent here.
//! The mark is made at each sync from the whole history and kept in memory only.

use super::state::StoreState;
use crate::notes::NoteRecord;
use std::collections::BTreeMap;

/// Whether `note` is the leaf the active pool holds at its position.
pub(crate) fn on_pool(committed: &BTreeMap<u64, [u8; 32]>, note: &NoteRecord) -> bool {
    committed.get(&note.leaf_index).map(crate::wallet::anchor::limbs) == Some(note.cm)
}

impl StoreState {
    /// Mark each held note the active pool does not hold at its position.
    pub(crate) fn mark_pool(&mut self, committed: &BTreeMap<u64, [u8; 32]>) {
        self.elsewhere =
            self.notes.values().filter(|n| !on_pool(committed, n)).map(|n| n.cm).collect();
    }

    /// Whether a held note counts here, as of the last mark.
    pub fn counts(&self, note: &NoteRecord) -> bool {
        !self.elsewhere.contains(&note.cm)
    }

    /// The held notes on the active pool, spent or not, as of the last mark.
    pub fn held_here(&self) -> Vec<&NoteRecord> {
        self.notes.values().filter(|n| self.counts(n)).collect()
    }
}

#[cfg(test)]
#[path = "pool_test.rs"]
mod pool_test;
