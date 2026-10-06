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

//! Keeping a wallet, and finding it again next boot.
//!
//! Two calls, at the two moments that matter: after a wallet comes into
//! existence, and when a fresh window has none. Everything under them is
//! best-effort by design. A machine with no TPM, or a store still staging
//! packages, must not stop the wallet working for this session; it only means
//! the wallet is for this session.

use nonos_libc::mk_time_millis;

use super::format::BLOB_LEN;
use super::keyring::{open_vault, seal_vault};
use super::load::{load_blob, load_file, Stored};
use super::path::VAULT_PATH;
use super::path::{ACCOUNTS_PATH, KIND_PATH, NETWORK_PATH, WORDS_PATH};
use super::read_judge::is_cleared;
use super::save::{save_blob, save_file};
use super::unsealed::Unsealed;
use crate::wallet::accounts::file;
use crate::wallet::ipc::{open_words, seal_words, WORDS_BLOB_LEN};

/// A year, matching the expiry every other keyring entry is stored with.
const LIFETIME_MS: u64 = 31_536_000_000;

/// Seal the wallet to this machine and put it on the disk, or say why not.
/// The caller reports a failure without treating the wallet as broken.
pub fn remember(port: u32, owner_pid: u32, wallet_id: u32) -> Result<(), Unsealed> {
    let blob = seal_vault(port, owner_pid, wallet_id).map_err(Unsealed::from_seal)?;
    save_blob(&blob)
}

/// Leave no vault a boot would restore: a cleared record of the vault's own
/// length (`read_judge::is_cleared`), since the store replaces a kept record
/// only at its length and a wallet cannot remove one. False when the store
/// would not take the write, so the last vault may still be there.
pub fn forget_vault() -> Result<(), Unsealed> {
    save_file(VAULT_PATH, &[0u8; BLOB_LEN])
}

/// Leave no recovery words a boot would open beside the next vault.
pub fn forget_words() -> Result<(), Unsealed> {
    save_file(WORDS_PATH, &[0u8; WORDS_BLOB_LEN])
}

/// What a restore found.
pub enum Recall {
    /// No vault on this disk, which is every machine that has never held a
    /// wallet and must not read as a failure.
    Nothing,
    /// The store has not answered yet. It stages packages for the first
    /// seconds of a boot and is deaf while it does, so this is the ordinary
    /// state of an early window and the caller must ask again rather than
    /// conclude the machine has no wallet.
    NotYet,
    /// Opened, and this is the wallet.
    Wallet(u32),
    /// A vault is there and this machine will not open it. The keyring says
    /// which: ENOENT when it could not derive a key, so the machine changed
    /// or has no TPM, and anything else when a key was derived and did not
    /// fit, so the vault belongs to another machine.
    Sealed { machine_changed: bool },
}

pub fn recall(port: u32, owner_pid: u32) -> Recall {
    let blob = match load_blob() {
        Stored::Blob(b) if is_cleared(&b) => return Recall::Nothing,
        Stored::Blob(b) => b,
        Stored::None => return Recall::Nothing,
        Stored::Unknown => return Recall::NotYet,
    };
    let now = mk_time_millis().max(0) as u64;
    match open_vault(port, owner_pid, now, now.saturating_add(LIFETIME_MS), &blob) {
        Ok(id) => Recall::Wallet(id),
        /* The keyring did not answer: nothing was opened or refused, so the
         * restore asks again rather than call the vault sealed. */
        Err(-11) => Recall::NotYet,
        Err(-2) => Recall::Sealed { machine_changed: true },
        Err(_) => Recall::Sealed { machine_changed: false },
    }
}

/* The recovery words beside the vault, sealed by the keyring. An account
 * from a private key has none, and that is not a failure. */
pub fn remember_words(port: u32, owner_pid: u32, wallet_id: u32) -> Result<(), Unsealed> {
    let blob = seal_words(port, owner_pid, wallet_id).map_err(Unsealed::from_seal)?;
    save_file(WORDS_PATH, &blob)
}

/* What opening the sealed words came to. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Words {
    /// The keyring holds the words again.
    Opened,
    /// There is no words file: the wallet came from a key, or never kept them.
    Absent,
    /// The store did not answer, or the keyring would not open them: ask again.
    Unread,
}

/* Open the sealed words beside account 0 again. Without them the account
 * still works; only further accounts and the shield's first opening wait. */
pub fn recall_words(port: u32, owner_pid: u32, wallet_id: u32) -> Words {
    let blob = match load_file::<WORDS_BLOB_LEN>(WORDS_PATH) {
        Stored::Blob(b) if is_cleared(&b) => return Words::Absent,
        Stored::Blob(b) => b,
        Stored::None => return Words::Absent,
        Stored::Unknown => return Words::Unread,
    };
    let now = mk_time_millis().max(0) as u64;
    match open_words(port, owner_pid, wallet_id, now, now.saturating_add(LIFETIME_MS), &blob) {
        Ok(()) => Words::Opened,
        Err(_) => Words::Unread,
    }
}

/* Write down whether the wallet came from a key. */
pub fn remember_kind(from_key: bool) -> Result<(), Unsealed> {
    save_file(KIND_PATH, &[from_key as u8])
}

/* Whether the wallet came from a key: None when the store did not answer,
 * the wallet is older than the record, or the byte is neither answer. */
pub fn recall_kind() -> Option<bool> {
    match load_file::<1>(KIND_PATH) {
        Stored::Blob([0]) => Some(false),
        Stored::Blob([1]) => Some(true),
        _ => None,
    }
}

pub fn remember_accounts(count: u8, open: u8) -> Result<(), Unsealed> {
    save_file(ACCOUNTS_PATH, &file::encode(count, open))
}

pub fn recall_accounts() -> Option<(u8, u8)> {
    match load_file::<{ file::FILE_LEN }>(ACCOUNTS_PATH) {
        Stored::Blob(b) => file::decode(&b),
        _ => None,
    }
}

/* Write down the network picked, on a boot that keeps anything. */
pub fn remember_network(sepolia: bool) -> Result<(), Unsealed> {
    save_file(NETWORK_PATH, &[sepolia as u8])
}

/* The network picked when last kept: Some(true) for Sepolia, None when none
 * was kept, the store did not answer, or the byte is neither answer. */
pub fn recall_network() -> Option<bool> {
    match load_file::<1>(NETWORK_PATH) {
        Stored::Blob([0]) => Some(false),
        Stored::Blob([1]) => Some(true),
        _ => None,
    }
}
