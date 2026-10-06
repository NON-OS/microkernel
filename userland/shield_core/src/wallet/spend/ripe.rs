//! When a new note may be spent: after 20 more leaves and six hours, since one moved soon after it
//! lands is linked to its deposit by timing. The owner can go early with a typed confirmation.

use crate::net::rpc::RawLog;
use std::collections::BTreeMap;

/// Leaves the pool must add after a note, and blocks of twelve seconds that make six hours.
pub const WAIT_LEAVES: u64 = 20;
pub const WAIT_BLOCKS: u64 = 1_800;

/// The block each leaf landed in, from `NoteCommitted`, whose third topic is the leaf index.
fn leaf_blocks(committed: &[RawLog]) -> BTreeMap<u64, u64> {
    let mut out = BTreeMap::new();
    for log in committed {
        if let Some(index) = log.topics.get(2).and_then(|t| t.get(24..32)) {
            let mut b = [0u8; 8];
            b.copy_from_slice(index);
            out.insert(u64::from_be_bytes(b), log.block);
        }
    }
    out
}

/// Which leaves are ripe at `head`, as a test a spend applies to each candidate note.
pub(crate) fn ripe(committed: &[RawLog], head: u64) -> impl Fn(u64) -> bool {
    let blocks = leaf_blocks(committed);
    let leaves = u64::try_from(committed.len()).unwrap_or(0);
    move |leaf| {
        let grown = leaf.saturating_add(WAIT_LEAVES) < leaves;
        let aged = blocks.get(&leaf).is_some_and(|b| b.saturating_add(WAIT_BLOCKS) <= head);
        grown && aged
    }
}

/// What the least ready of `leaves` still waits for: leaves to land, then blocks to pass.
pub fn waiting(committed: &[RawLog], head: u64, leaves: &[u64]) -> (u64, u64) {
    let blocks = leaf_blocks(committed);
    let count = u64::try_from(committed.len()).unwrap_or(0);
    leaves.iter().fold((0, 0), |(most_leaves, most_blocks), leaf| {
        let grown = count.saturating_sub(leaf.saturating_add(1));
        let to_land = WAIT_LEAVES.saturating_sub(grown);
        let aged = blocks.get(leaf).map_or(0, |b| head.saturating_sub(*b));
        (most_leaves.max(to_land), most_blocks.max(WAIT_BLOCKS.saturating_sub(aged)))
    })
}
