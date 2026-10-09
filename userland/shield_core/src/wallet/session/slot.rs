//! One account of the wallet: its shield keys, its public account and its own sealed note log.

use crate::custody::Seed;
use crate::error::{CustodyError, WalletError};
use crate::evm::EvmAccount;
use crate::keys::Account;
use crate::store::{NoteLog, StoreState};
use crate::wallet::paths::Paths;

/// Everything account `index` of the seed holds while the wallet is unlocked.
pub(crate) struct Slot {
    pub(super) account: Account,
    pub(super) evm: EvmAccount,
    pub(super) log: NoteLog,
    pub(super) state: StoreState,
}

impl Slot {
    /// Derive account `index` and replay its note log, each sealed under its own store key.
    pub(super) fn open(paths: &Paths, seed: &Seed, index: u32) -> Result<Slot, WalletError> {
        let evm = EvmAccount::at(seed.bytes(), index).ok_or(CustodyError::Mnemonic)?;
        Slot::with_public(paths, seed, index, evm)
    }

    /// Account 0 of a wallet from a private key: its public account is that key.
    pub(super) fn imported(
        paths: &Paths,
        seed: &Seed,
        key: &[u8; 32],
    ) -> Result<Slot, WalletError> {
        let evm = EvmAccount::from_key(key).ok_or(CustodyError::KeyShape)?;
        Slot::with_public(paths, seed, 0, evm)
    }

    /// Account `index`, whose note log is known not to be on the disk: nothing is read.
    pub(super) fn unwritten(paths: &Paths, seed: &Seed, index: u32) -> Result<Slot, WalletError> {
        let evm = EvmAccount::at(seed.bytes(), index).ok_or(CustodyError::Mnemonic)?;
        let account = Account::at(seed, index)?;
        let (log, state) = NoteLog::unwritten_account(paths.notes_of(index), seed, index)?;
        Ok(Slot { account, evm, log, state })
    }

    fn with_public(
        paths: &Paths,
        seed: &Seed,
        index: u32,
        evm: EvmAccount,
    ) -> Result<Slot, WalletError> {
        let account = Account::at(seed, index)?;
        let (log, state) = NoteLog::open_account(paths.notes_of(index), seed, index)?;
        Ok(Slot { account, evm, log, state })
    }

    pub(crate) fn account(&self) -> &Account {
        &self.account
    }

    pub(crate) fn evm(&self) -> &EvmAccount {
        &self.evm
    }

    pub(crate) fn state(&self) -> &StoreState {
        &self.state
    }
}
