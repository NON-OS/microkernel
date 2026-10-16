//! What the folded state answers: its history, deposits on their way, notes held, the cursor.

use super::StoreState;
use crate::notes::NoteRecord;

impl StoreState {
    /// What the wallet did with the pool, oldest first.
    pub fn activity(&self) -> &[crate::store::activity::Activity] {
        &self.activity
    }

    /// Deposits sent and not yet seen stored in the pool.
    pub fn pending_deposits(&self) -> Vec<crate::notes::NotePlaintext> {
        self.pending.values().cloned().collect()
    }

    /// Every note held, spent or not, for matching published nullifiers.
    pub fn held(&self) -> Vec<&NoteRecord> {
        self.notes.values().collect()
    }

    /// Where the next scan should start.
    pub fn cursor(&self) -> u64 {
        self.cursor
    }
}
