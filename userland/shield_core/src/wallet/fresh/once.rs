//! OP_FRESH_ADDRESS mode 1, "once" (plan/27, v1.4.52): an address no order
//! or channel shares. Its index is past the account count and past every
//! index handed out, and it is kept as handed before it is answered. The
//! addresses ahead are derived at unlock, while the seed is at hand, so
//! answering needs neither the seed nor the session.

use std::path::Path;

use super::handed::{highest_handed, keep_handed, MAX_HANDED};
use crate::error::WalletError;
use crate::evm::EvmAccount;

/// How many addresses ahead an unlock derives; a lock and unlock derives
/// more once they are used.
pub const AHEAD: u32 = 64;

/// The first index mode 1 may answer: past `count` and every handed index.
pub fn first_once(count: u32, highest: Option<u32>) -> u32 {
    highest.map_or(count, |h| count.max(h.saturating_add(1)))
}

/// The AHEAD addresses from `start`, derived from the seed.
pub fn derive_ahead(seed: &[u8; 64], start: u32) -> Vec<(u32, [u8; 20])> {
    let end = start.saturating_add(AHEAD).min(MAX_HANDED.saturating_add(1));
    (start..end).filter_map(|i| EvmAccount::at(seed, i).map(|a| (i, a.address()))).collect()
}

/// The address mode 1 answers from `ahead`, or None once they are used.
pub fn pick_once(
    count: u32,
    highest: Option<u32>,
    ahead: &[(u32, [u8; 20])],
) -> Option<(u32, [u8; 20])> {
    let from = first_once(count, highest);
    ahead.iter().find(|(i, _)| *i >= from).copied()
}

/// Mode 1: pick, keep as handed, then answer.
pub fn fresh_once(
    dir: &Path,
    count: u32,
    ahead: &[(u32, [u8; 20])],
) -> Result<Option<[u8; 20]>, WalletError> {
    let Some((index, address)) = pick_once(count, highest_handed(dir), ahead) else {
        return Ok(None);
    };
    keep_handed(dir, index, &address)?;
    Ok(Some(address))
}
