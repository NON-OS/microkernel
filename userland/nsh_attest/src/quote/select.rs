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

//! The selection a quote must make, and the digest it must sign.

use super::SELECT_BYTES;
use nonos_hash::Sha256;

/// PCR indices to a selection bitmap, low bit of each byte first. An index
/// past the bank sets nothing, so it cannot alias another PCR.
pub fn bitmap(pcrs: &[u8]) -> [u8; SELECT_BYTES] {
    let mut bits = [0u8; SELECT_BYTES];
    for &p in pcrs {
        if let Some(b) = bits.get_mut(p as usize / 8) {
            *b |= 1 << (p % 8);
        }
    }
    bits
}

/// `pcrDigest` for one bank: SHA-256 over the values, in selection order.
pub fn pcr_digest(values: &[&[u8; 32]]) -> [u8; 32] {
    let mut h = Sha256::new();
    for v in values {
        h.update(&v[..]);
    }
    h.finalize()
}
