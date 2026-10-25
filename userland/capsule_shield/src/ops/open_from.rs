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

//! Opening from the recovery words or from a key, the private address shown first.

use nox_shield_core::ffi::{address_of_key, address_of_words};

use super::open::open_with;
use super::{refuse, Answer};

pub fn open_words(f: &[&str]) -> Answer {
    let [address, words] = f else { return refuse("Name the account and its words.") };
    let list: Vec<String> = words.split(' ').filter(|w| !w.is_empty()).map(String::from).collect();
    /* The private address first, from the words alone: shown before the store exists. */
    crate::steps::begin();
    crate::steps::step("deriving the private address");
    if let Some(account) = public(address) {
        crate::steps::preview(address_of_words(&list, account).ok().flatten());
    }
    open_with(address, |w| w.restore(list).map_err(|e| e.to_string()))
}

pub fn open_key(f: &[&str]) -> Answer {
    let [address, key] = f else { return refuse("Name the account and its key.") };
    let key = String::from(*key);
    crate::steps::begin();
    crate::steps::step("deriving the private address");
    if let Some(account) = public(address) {
        crate::steps::preview(address_of_key(&key, account).ok().flatten());
    }
    open_with(address, move |w| w.restore_from_key(key).map_err(|e| e.to_string()))
}

/// The 0x account the wallet named, as bytes.
fn public(address: &str) -> Option<[u8; 20]> {
    let hex = address.trim().trim_start_matches("0x");
    if hex.len() != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}
