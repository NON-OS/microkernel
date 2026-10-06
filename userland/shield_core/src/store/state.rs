//! What the rows fold up to, keyed by commitment. The balance comes only from notes held here.

use super::row::Row;
use crate::notes::NoteRecord;
use std::collections::BTreeMap;

/// What the wallet holds, as the rows fold up to.
#[derive(Default)]
pub struct StoreState {
    pub(super) notes: BTreeMap<[u64; 4], NoteRecord>,
    cursor: u64,
    /// Deposits sent and not yet seen stored, by the commitment they will have.
    pending: BTreeMap<[u64; 4], crate::notes::NotePlaintext>,
    /// Notes held from another pool, marked at each sync and never written.
    pub(super) elsewhere: std::collections::BTreeSet<[u64; 4]>,
    /// What the wallet did with the pool, in the order it was recorded.
    activity: Vec<crate::store::activity::Activity>,
}

/// One coin's balance in its note units, split into spendable now and in flight.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Balance {
    pub coin: crate::net::asset::Coin,
    pub spendable: u64,
    pub pending: u64,
    pub note_count: u32,
}

impl StoreState {
    pub(super) fn empty() -> StoreState {
        StoreState::default()
    }

    /// Fold one row in. A status row for an unseen note is ignored, it may await a rescan.
    pub(crate) fn apply(&mut self, row: Row) {
        match row {
            Row::Found(record) => {
                self.pending.remove(&record.cm);
                self.notes.insert(record.cm, record);
            }
            Row::Status { cm, status } => {
                if let Some(record) = self.notes.get_mut(&cm) {
                    record.status = status;
                }
            }
            Row::Cursor(at) => self.cursor = self.cursor.max(at),
            Row::Activity(a) => self.activity.push(a),
            Row::Deposit(plain) => {
                let cm = crate::notes::commitment(&crate::prover::pool_hasher(), &plain.note());
                let key = [cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()];
                if !self.notes.contains_key(&key) {
                    self.pending.insert(key, plain);
                }
            }
        }
    }

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
