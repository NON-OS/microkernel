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

//! The recovery words kept sealed beside the vault, and opening them again.

use nonos_libc::mk_time_millis;

use super::super::load::{load_file, Stored};
use super::super::path::WORDS_PATH;
use super::super::read_judge::is_cleared;
use super::super::save::save_file;
use super::super::unsealed::Unsealed;
use super::LIFETIME_MS;
use crate::wallet::ipc::{open_words, seal_words, WORDS_BLOB_LEN};

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
