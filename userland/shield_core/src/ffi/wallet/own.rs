//! What a review needs before a private transfer between two accounts of one wallet: whether the
//! address is one of this wallet's accounts, and whether the value arrived so recently that the
//! timing could link the two.

use super::Wallet;
use crate::error::WalletError;
use crate::keys::parse_receiving_address;
use crate::notes::NoteStatus;

/// Notes among the newest this many leaves of the pool count as having just arrived.
const RECENT_LEAVES: u64 = 16;

impl Wallet {
    /// The account of this wallet a `nox1` address belongs to, or none for another wallet.
    pub fn own_account_of(&self, address: String) -> Result<Option<u32>, WalletError> {
        let (spend_pk, _) = parse_receiving_address(&address)?;
        self.with(|s| {
            (0..s.count()).find(|i| {
                s.at(*i).is_some_and(|slot| slot.account().address().spend_pk == spend_pk)
            })
        })
    }

    /// Whether the active account holds a note among the pool's newest leaves at the last sync.
    pub fn arrived_recently(&self) -> Result<bool, WalletError> {
        self.with(|s| {
            s.leaves_seen > 0
                && s.state().held().iter().any(|n| {
                    n.status == NoteStatus::Unspent
                        && n.leaf_index.saturating_add(RECENT_LEAVES) >= s.leaves_seen
                })
        })
    }
}
