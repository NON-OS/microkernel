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

//! A family's private randomness and its seal's handle, from the kernel's
//! RNG. A family that cannot have them is not started.

use crate::crypto::util::rng::global::get_random_bytes_secure;

/// Goldilocks p, the bound every salt word sits below.
const P: u64 = 0xFFFF_FFFF_0000_0001;

fn words(r: &[u8; 32]) -> [u64; 4] {
    core::array::from_fn(|i| {
        let mut w = [0u8; 8];
        w.copy_from_slice(&r[8 * i..8 * i + 8]);
        u64::from_le_bytes(w)
    })
}

/// K0: 32 fresh bytes per family, never output, never a boot nonce.
pub(crate) fn trace_start() -> Option<[u8; 32]> {
    get_random_bytes_secure().ok()
}

/// Four field words, drawn by rejection rather than reduced, so each is
/// uniform below p. A word at or above p comes once in 2^32 draws.
pub(crate) fn salt() -> Option<[u64; 4]> {
    for _ in 0..4 {
        let w = words(&get_random_bytes_secure().ok()?);
        if w.iter().all(|&v| v < P) {
            return Some(w);
        }
    }
    None
}

/// A random non-zero handle, so a seal number says nothing about how many
/// families this machine ran.
pub(crate) fn seal_no() -> Option<u64> {
    let w = words(&get_random_bytes_secure().ok()?);
    w.into_iter().find(|&v| v != 0)
}
