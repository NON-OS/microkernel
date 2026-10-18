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

//! The small records kept beside the vault: its kind, accounts and network.

use super::super::load::{load_file, Stored};
use super::super::path::{ACCOUNTS_PATH, KIND_PATH, NETWORK_PATH};
use super::super::save::save_file;
use super::super::unsealed::Unsealed;
use crate::wallet::accounts::file;

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
