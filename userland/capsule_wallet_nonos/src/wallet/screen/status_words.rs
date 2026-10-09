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

//! The status line's words for the account's readings and its keeping, pure
//! so wallet_proofs holds them.

use alloc::string::String;

/// Whether the account's balances are being read, were read, or are not.
/// `None` before a wallet exists, when there is nothing to read.
pub fn reading(wallet: bool, refreshing: bool, read: bool) -> Option<&'static str> {
    if !wallet {
        return None;
    }
    Some(if refreshing {
        "reading"
    } else if read {
        "synced"
    } else {
        "not read"
    })
}

/// Where the wallet is kept. `None` before a wallet exists.
pub fn kept(wallet: bool, sealed: bool) -> Option<&'static str> {
    wallet.then_some(if sealed { "sealed" } else { "RAM only" })
}

/// What the balances say when not one of them has been read: one line in
/// place of a dash for each coin.
pub fn unread_balances(refreshing: bool, online: bool) -> &'static str {
    if refreshing {
        "reading the network"
    } else if online {
        "not read yet"
    } else {
        "offline, balance not checked"
    }
}

/// A status sentence as a screen sentence: its first letter capitalised.
pub fn capital(note: &str) -> String {
    let mut out = String::with_capacity(note.len() + 1);
    let mut chars = note.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
    }
    out.push_str(chars.as_str());
    out.push('.');
    out
}
