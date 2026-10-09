//! Marking, in one account, the notes that are not on the active pool.

use super::Session;
use std::collections::BTreeMap;

impl Session {
    /// Mark the notes of account `index` against the active pool's leaves.
    pub(crate) fn mark_pool_at(&mut self, index: u32, committed: &BTreeMap<u64, [u8; 32]>) {
        let slot = match index.checked_sub(1) {
            None => Some(&mut self.first),
            Some(i) => usize::try_from(i).ok().and_then(|i| self.more.get_mut(i)),
        };
        if let Some(slot) = slot {
            slot.state.mark_pool(committed);
        }
    }
}
