//! What to do next for a published spend, from the clock and the chain alone.

use super::landed::Seen;
use super::record::Published;

/// Republish a spend that has not landed this long after its latest publication.
pub const REPUBLISH_AFTER: u64 = 15 * 60;
/// Offer the owner to settle it this long after its first publication.
pub const SELF_SETTLE_AFTER: u64 = 30 * 60;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Next {
    Landed(Option<[u8; 32]>),
    Elsewhere,
    Wait,
    Republish,
}

/// The next step for a spend its owner already sent the settlement for: followed to the block,
/// never handed to a lander again, and not offered again.
pub fn next_settling(now: u64, record: &Published, seen: Seen) -> (Next, bool) {
    match next(now, record, seen) {
        (Next::Republish, _) | (Next::Wait, _) => (Next::Wait, false),
        done => done,
    }
}

/// The next step at `now`, and whether settling it from the public account is offered.
pub fn next(now: u64, record: &Published, seen: Seen) -> (Next, bool) {
    let step = match seen {
        Seen::Landed(tx) => return (Next::Landed(tx), false),
        Seen::Elsewhere => return (Next::Elsewhere, false),
        Seen::Neither if now >= record.last.saturating_add(REPUBLISH_AFTER) => Next::Republish,
        Seen::Neither => Next::Wait,
    };
    (step, now >= record.first.saturating_add(SELF_SETTLE_AFTER))
}
