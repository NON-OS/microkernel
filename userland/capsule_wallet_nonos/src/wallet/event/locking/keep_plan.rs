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

pub use super::kept_status::kept_status;
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
    Landed,
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
        Ok(()) => Kept::Landed,
        Err(why) => Kept::Lost(why),
    }
}
