//! Reading the folded store. Spendable notes come oldest first, which keeps newer anonymity
//! sets intact for longer. The balance saturates, so a corrupt store never panics.

use super::state::{Balance, StoreState};
use crate::net::asset::Asset;
use crate::notes::{NoteRecord, NoteStatus};

impl StoreState {
    /// The notes that can be spent, oldest position first.
    pub fn spendable(&self) -> Vec<&NoteRecord> {
        let mut out: Vec<&NoteRecord> =
            self.held_here().into_iter().filter(|n| n.status == NoteStatus::Unspent).collect();
        out.sort_by_key(|n| n.leaf_index);
        out
    }

    /// One asset's balance, summed from its own notes only and never from a server.
    pub fn balance(&self, asset: &Asset) -> Balance {
        let mut spendable = 0u64;
        let mut pending = 0u64;
        let mut count = 0u32;
        for n in self.held_here().into_iter().filter(|n| n.plain.asset_id == asset.id) {
            match n.status {
                NoteStatus::Unspent => spendable = spendable.saturating_add(n.plain.value),
                NoteStatus::Pending => pending = pending.saturating_add(n.plain.value),
                NoteStatus::Spent => continue,
            }
            count = count.saturating_add(1);
        }
        Balance { coin: asset.coin, spendable, pending, note_count: count }
    }
}
