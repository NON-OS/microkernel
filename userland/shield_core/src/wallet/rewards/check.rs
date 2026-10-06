//! Whether the registry would take a link or an unlink now, named before anything is signed or
//! sent. A link in force is never sent again, which would move the epoch it counts from.

use super::calls::{EPOCH, GENESIS};
use super::read::LinkState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkRefusal {
    /// The testnet address is linked to another mainnet address, to be unlinked first.
    TestnetTaken,
    ChangedThisEpoch,
    AlreadyLinked,
    NotLinked,
}

/// Whether `mainnet` may link to `testnet` now.
pub fn may_link(
    state: &LinkState,
    mainnet: &[u8; 20],
    testnet: &[u8; 20],
) -> Result<(), LinkRefusal> {
    if state.linked_to.is_some_and(|m| m != *mainnet) {
        return Err(LinkRefusal::TestnetTaken);
    }
    if state.linked_to == Some(*mainnet) && state.testnet == Some(*testnet) {
        return Err(LinkRefusal::AlreadyLinked);
    }
    if state.started && state.nonce != 0 && state.changed_in == state.epoch {
        return Err(LinkRefusal::ChangedThisEpoch);
    }
    Ok(())
}

/// The testnet side may always leave a link it has.
pub fn may_unlink(state: &LinkState) -> Result<(), LinkRefusal> {
    state.linked_to.map(|_| ()).ok_or(LinkRefusal::NotLinked)
}

/// The epoch a link made now counts from, and when it starts, in seconds since 1970.
pub fn counts_from(state: &LinkState) -> (u64, u64) {
    let epoch = if state.started { state.epoch.saturating_add(1) } else { 0 };
    (epoch, GENESIS.saturating_add(epoch.saturating_mul(EPOCH)))
}
