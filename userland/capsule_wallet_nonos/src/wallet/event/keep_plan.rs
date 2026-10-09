// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The order a new wallet's records are written in, and what a failure
//! leaves behind.
//!
//! The vault is the record a boot restores from, so it is cleared first and
//! written last. A wallet is never restored with the words, kind or account
//! list of the one before it: a failure or a power cut part way leaves no
//! vault at all, and the next boot is a machine with no wallet, never the
//! last wallet's key beside this one's words.

use crate::wallet::vault::Unsealed;

/// The writes a keep is made of.
pub trait Records {
    /// Make the vault on disk unreadable as a vault.
    fn clear_vault(&mut self) -> Result<(), Unsealed>;
    /// Whether this wallet came from a key.
    fn kind(&mut self, from_key: bool) -> Result<(), Unsealed>;
    /// Seal the recovery words and store them.
    fn words(&mut self) -> Result<(), Unsealed>;
    /// Make the words file unreadable as words: a wallet from a key has none.
    fn clear_words(&mut self) -> Result<(), Unsealed>;
    /// The account list.
    fn accounts(&mut self) -> Result<(), Unsealed>;
    /// Seal the account key and store it.
    fn vault(&mut self) -> Result<(), Unsealed>;
}

/// What a keep came to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kept {
    /// Every record landed: the next boot restores this wallet.
    Kept,
    /// The last vault could not be cleared, so whatever was stored is still
    /// on the disk and the next boot restores that, not this one.
    LastStays(Unsealed),
    /// The last vault is gone and this one did not land: the next boot has
    /// no wallet.
    Lost(Unsealed),
    /// A boot that keeps nothing past power off: nothing was written, the
    /// disk is as it was, and this wallet is in memory only.
    Live,
}

pub fn keep_records(records: &mut impl Records, from_key: bool) -> Kept {
    if let Err(why) = records.clear_vault() {
        return Kept::LastStays(why);
    }
    let rest = records
        .kind(from_key)
        .and_then(|()| if from_key { records.clear_words() } else { records.words() })
        .and_then(|()| records.accounts())
        .and_then(|()| records.vault());
    match rest {
        Ok(()) => Kept::Kept,
        Err(why) => Kept::Lost(why),
    }
}

/// The status line for a keep that did not land, or None when it did.
pub fn kept_status(kept: Kept, from_key: bool) -> Option<&'static [u8]> {
    let s: &'static [u8] = match (kept, from_key) {
        (Kept::Kept, _) => return None,
        (Kept::Live, false) => b"this is a live session: nothing is kept past power off, so this wallet is gone then; write down the phrase",
        (Kept::Live, true) => b"this is a live session: nothing is kept past power off, so this wallet is gone then; keep the private key",
        (Kept::LastStays(Unsealed::DiskFull), false) => b"the disk is full, so this wallet is gone at reboot and any wallet stored before stays: write down the phrase",
        (Kept::LastStays(Unsealed::DiskFull), true) => b"the disk is full, so this wallet is gone at reboot and any wallet stored before stays: keep the private key",
        (Kept::LastStays(_), false) => b"the store would not take this wallet, so it is gone at reboot and any wallet stored before stays: write down the phrase",
        (Kept::LastStays(_), true) => b"the store would not take this wallet, so it is gone at reboot and any wallet stored before stays: keep the private key",
        (Kept::Lost(Unsealed::NoMachineKey), false) => b"no machine key to seal under, so this wallet is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::NoMachineKey), true) => b"no machine key to seal under, so this wallet is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::Keyring), false) => b"the keyring would not seal this wallet, so it is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::Keyring), true) => b"the keyring would not seal this wallet, so it is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::DiskFull), false) => b"the disk is full, so this wallet is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::DiskFull), true) => b"the disk is full, so this wallet is gone at reboot: keep the private key",
        (Kept::Lost(Unsealed::Store), false) => b"the store would not keep this wallet, so it is gone at reboot: write down the phrase",
        (Kept::Lost(Unsealed::Store), true) => b"the store would not keep this wallet, so it is gone at reboot: keep the private key",
    };
    Some(s)
}
