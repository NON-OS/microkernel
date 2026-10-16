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

//! Choosing the account the wallet named among the store's accounts.

use nox_shield_core::ffi::Wallet;

use super::refuse;

/* The wallet's further accounts are m/44'/60'/0'/0/i of the same words, as
 * shield-core's are: make the one the wallet named the active account,
 * adding accounts up to it. A store whose accounts never reach it belongs
 * to another phrase. */
const MAX_ACCOUNTS: usize = 8;

pub(super) fn pick_account(wallet: &Wallet, address: &str) -> Result<String, String> {
    for _ in 0..MAX_ACCOUNTS {
        let all = wallet.accounts().map_err(|e| e.to_string())?;
        if let Some(a) = all.iter().find(|a| a.public_address.eq_ignore_ascii_case(address)) {
            if !a.active {
                wallet.select_account(a.index).map_err(|e| e.to_string())?;
            }
            return Ok(a.public_address.clone());
        }
        if all.len() >= MAX_ACCOUNTS || wallet.from_key().unwrap_or(true) {
            break;
        }
        wallet.add_account().map_err(|e| e.to_string())?;
    }
    refuse("The shield store here belongs to another account. Nothing was opened.")
}
