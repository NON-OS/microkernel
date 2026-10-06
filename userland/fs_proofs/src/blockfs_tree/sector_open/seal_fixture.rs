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

//! Sectors sealed as `cryptoblock::seal` seals them, by the kernel's AEAD.

use super::constants::{AAD_PREFIX, NONCE_BYTES, SECTOR_BYTES};
use crate::crypto::chacha20poly1305::aead_encrypt;

pub fn fill(seed: &mut u64, out: &mut [u8]) {
    for b in out {
        *seed =
            seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        *b = (*seed >> 33) as u8;
    }
}

pub fn aad(lba: u64) -> [u8; 24] {
    let mut aad = [0u8; 24];
    aad[..16].copy_from_slice(AAD_PREFIX);
    aad[16..].copy_from_slice(&lba.to_le_bytes());
    aad
}

/// A sector as `cryptoblock::seal` lays it out: nonce, then sealed bytes and tag.
pub fn sealed(seed: &mut u64, key: &[u8; 32], lba: u64, plain: &[u8]) -> [u8; SECTOR_BYTES] {
    let mut nonce = [0u8; NONCE_BYTES];
    fill(seed, &mut nonce);
    let body = aead_encrypt(key, &nonce, &aad(lba), plain).unwrap();
    let mut sector = [0u8; SECTOR_BYTES];
    sector[..NONCE_BYTES].copy_from_slice(&nonce);
    sector[NONCE_BYTES..].copy_from_slice(&body);
    sector
}
