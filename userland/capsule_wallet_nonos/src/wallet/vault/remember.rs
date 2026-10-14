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

use super::format::BLOB_LEN;
use super::keyring::seal_vault;
use super::path::VAULT_PATH;
use super::path::WORDS_PATH;
use super::save::{save_blob, save_file};
use super::unsealed::Unsealed;
use crate::wallet::ipc::WORDS_BLOB_LEN;

#[path = "remember_prefs.rs"]
mod remember_prefs;
#[path = "remember_recall.rs"]
mod remember_recall;
#[path = "remember_words.rs"]
mod remember_words;
pub use remember_prefs::{
    recall_accounts, recall_kind, recall_network, remember_accounts, remember_kind,
    remember_network,
};
pub use remember_recall::{recall, Recall};
pub use remember_words::{recall_words, remember_words, Words};

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
