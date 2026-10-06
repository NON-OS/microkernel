//! The wallet's notes, from three pool events and nothing else.
//! `NoteCommitted` names each leaf, `OutputNote` carries the X-Wing blob of a settlement output,
//! and `NullifierSpent` retires a note. No server is asked anything.

use super::chain_viewer::scan_viewer;
use super::logs::Log;
use crate::keys::Account;
use crate::notes::{NotePlaintext, NoteRecord};

/// What one pass over the pool's logs found.
#[derive(Default)]
pub struct ChainScan {
    /// Notes another wallet paid to this one, each opened and checked.
    pub received: Vec<NoteRecord>,
    /// This wallet's own deposits the pool has now stored.
    pub deposited: Vec<NoteRecord>,
    /// Commitments of this wallet's notes whose nullifier the pool published.
    pub spent: Vec<[u64; 4]>,
}

/// Read this wallet's notes from the logs. `pending` are sent deposits not yet seen stored,
/// and `held` are notes already held, so a published nullifier can be matched to them.
pub fn scan_chain(
    account: &Account,
    committed: &[Log],
    outputs: &[Log],
    nullifiers: &[Log],
    pending: &[NotePlaintext],
    held: &[&NoteRecord],
) -> ChainScan {
    scan_viewer(&account.viewer(), committed, outputs, nullifiers, pending, held)
}
