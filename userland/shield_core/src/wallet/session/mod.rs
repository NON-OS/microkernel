//! A live wallet: the accounts of an unlocked seed, each with its own store. Locking drops it.
//! Every call about keys, notes or the public account reaches the active account.

mod accounts;
pub(crate) mod book;
mod candidates;
mod install;
pub(super) mod open;
mod pool;
mod public;
mod restore;
pub(crate) mod slot;

use crate::error::StoreError;
use crate::keys::{Account, Address};
use crate::store::{Balance, Row, StoreState};
use slot::Slot;

pub struct Session {
    pub(super) first: Slot,
    pub(super) more: Vec<Slot>,
    /// Always an index `at` answers, which `select` checks before it changes.
    pub(super) active: u32,
    /// The view-only accounts, which are never active: they have no send.
    pub(crate) watching: crate::wallet::watch::Watching,
    /// Accounts a restore derived and a search has not yet looked at. Empty after an unlock.
    pub(super) candidates: Vec<Slot>,
    pub(crate) leaves_seen: u64,
    /// From a private key: one account, no words.
    pub(crate) imported: bool,
    /// The public address of the next account, unused, for a withdrawal. None for a key wallet.
    pub(crate) next_public: Option<[u8; 20]>,
}

impl Session {
    pub fn address(&self) -> Address {
        self.slot().account.address()
    }

    pub fn balances(&self) -> Vec<Balance> {
        let state = &self.slot().state;
        crate::net::pool::ACTIVE.assets.iter().map(|a| state.balance(a)).collect()
    }

    pub fn evm(&self) -> &crate::evm::EvmAccount {
        &self.slot().evm
    }

    pub fn account(&self) -> &Account {
        &self.slot().account
    }

    pub fn state(&self) -> &StoreState {
        &self.slot().state
    }

    pub fn cursor(&self) -> u64 {
        self.slot().state.cursor()
    }

    /// Disk first, or a spent note could return after a restart.
    pub(crate) fn record(&mut self, row: Row) -> Result<(), StoreError> {
        let index = self.active;
        self.record_to(index, row)
    }
}
