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

//! Where an account's store lives: one folder per 0x account.

use super::refuse;

/// One store per 0x account, so a second wallet on this machine never
/// opens, or overwrites, the first one's notes.
///
/// Kept in memory on every boot. The store is files the phones' core writes
/// with std::fs, and vfs keeps only what a capsule hands to OP_STORE_PERSIST,
/// one fixed record at a time: these files are in its RAM tree and gone at
/// power off, whatever volume is mounted. So the store says so (`kept` is
/// "memory") and its file key is wrapped under a key drawn for this boot,
/// never under the machine key, which would claim a keeping it does not
/// have. The words open it again on the next boot, and a sync finds the
/// notes on chain.
const ROOT_LIVE: &str = "/run/shield";

pub(super) fn dir_of(address: &str) -> Result<String, String> {
    let hex = address.trim().trim_start_matches("0x").to_ascii_lowercase();
    if hex.len() != 40 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return refuse("The wallet named no account.");
    }
    Ok(format!("{ROOT_LIVE}/{hex}"))
}
