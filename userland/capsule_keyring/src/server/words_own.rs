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

//! Whether a wallet's words are an account's own, pure but for the curve, so
//! the host proves the very same source.

use nonos_hd::bip39::seed_from_words;
use nonos_hd::{derive_eth_key_at, wipe};

/// The highest account a wallet's words derive (m/44'/60'/0'/0/n).
pub const MAX_ACCOUNT_INDEX: u32 = 7;

/*
 * Whether `indices` derive `key` as one of the wallet's accounts. Sealed
 * words offered back are kept beside an account only when they are that
 * account's own: words beside the wrong key would give the shield another
 * wallet's private address, and further accounts another wallet's keys.
 */
pub fn words_derive(indices: &[u16], key: &[u8; 32]) -> bool {
    let mut seed = [0u8; 64];
    if !seed_from_words(indices, b"", &mut seed) {
        return false;
    }
    let mut found = false;
    for index in 0..=MAX_ACCOUNT_INDEX {
        let mut k = [0u8; 32];
        if derive_eth_key_at(&seed, pubkey, index, &mut k) {
            found |= same(&k, key);
        }
        wipe(&mut k);
    }
    wipe(&mut seed);
    found
}

/* Every byte compared, whatever the first difference. */
fn same(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a.iter().zip(b).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
}

fn pubkey(sk: &[u8; 32]) -> Option<[u8; 65]> {
    let mut out = [0u8; 65];
    (crate::server::secp::pubkey(sk, &mut out) == 65).then_some(out)
}
