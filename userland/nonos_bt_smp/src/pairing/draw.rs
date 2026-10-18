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

//! What pairing draws from the caller's random source, with each retry
//! bounded so a broken source ends pairing instead of spinning.

use super::io::Random;
use crate::ecdh::KeyPair;
use crate::error::SmpError;

/// A secret is out of range with odds near 2^-32, so eight misses mean the
/// source is broken.
const SECRET_TRIES: usize = 8;
/// Each draw lands below 10^6 with odds 0.95, so sixteen misses mean the
/// same.
const PASSKEY_TRIES: usize = 16;

pub(super) fn key_pair(rng: &mut dyn Random) -> Result<KeyPair, SmpError> {
    for _ in 0..SECRET_TRIES {
        let mut s = [0u8; 32];
        rng.fill(&mut s);
        let pair = KeyPair::from_secret(s);
        // SAFETY: `s` is a live, aligned local; the write keeps the copy
        // of the secret from lingering on the stack.
        unsafe { core::ptr::write_volatile(&mut s, [0; 32]) };
        match pair {
            Err(SmpError::BadSecret) => continue,
            other => return other,
        }
    }
    Err(SmpError::BadSecret)
}

pub(super) fn nonce(rng: &mut dyn Random) -> [u8; 16] {
    let mut n = [0u8; 16];
    rng.fill(&mut n);
    n
}

/// Six random decimal digits (Vol 3, Part H, 2.3.5.3). Twenty bits are
/// drawn and values from 10^6 up thrown away, so every passkey is equally
/// likely; taking the value modulo 10^6 would favour the low ones.
pub(super) fn passkey(rng: &mut dyn Random) -> Option<u32> {
    (0..PASSKEY_TRIES).find_map(|_| {
        let mut b = [0u8; 4];
        rng.fill(&mut b);
        Some(u32::from_le_bytes(b) & 0x000f_ffff).filter(|&v| v < 1_000_000)
    })
}
