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

//! The ids every track shares, each `keccak256(abi.encode(TAG,
//! ...))` with its tag from `nonos_nox_config`. Every field is a whole
//! 32-byte word, so `abi.encode` is the words laid end to end.

use nonos_nox_config::{BOND_TAG, LISTING_TAG, RELEASE_TAG};

use super::hashers::Keccak;

fn words(k: &impl Keccak, w: [&[u8; 32]; 3]) -> [u8; 32] {
    let mut pre = [0u8; 96];
    for (i, word) in w.iter().enumerate() {
        pre[32 * i..32 * (i + 1)].copy_from_slice(*word);
    }
    k.keccak256(&pre)
}

/// `publisher_key` is the listing's Ed25519 key; `app_id` the market id,
/// such as "linux.qwenchat".
pub fn listing_id(k: &impl Keccak, publisher_key: &[u8; 32], app_id: &str) -> [u8; 32] {
    words(k, [&LISTING_TAG, publisher_key, &k.keccak256(app_id.as_bytes())])
}

/// `version` is the release's `release_id` string.
pub fn release_id(k: &impl Keccak, listing_id: &[u8; 32], version: &str) -> [u8; 32] {
    words(k, [&RELEASE_TAG, listing_id, &k.keccak256(version.as_bytes())])
}

pub fn bond_id(k: &impl Keccak, payee: &[u8; 20], publisher_key: &[u8; 32]) -> [u8; 32] {
    let mut who = [0u8; 32];
    who[12..].copy_from_slice(payee);
    words(k, [&BOND_TAG, &who, publisher_key])
}
