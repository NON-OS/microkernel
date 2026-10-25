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

//! HKDF-SHA-256, RFC 5869, computed in the caller.

use super::hmac_sha256::tag_into;
use super::traffic_keys::wipe;

/*
 * Each HMAC here used to be its own syscall and kernel round trip to the
 * crypto pool: nine of them for the handshake keys and eight more for the
 * application keys, on every attempt at a handshake.
 */
/// The pseudorandom key, written into `prk` where the caller keeps it.
pub fn extract(salt: &[u8], ikm: &[u8], prk: &mut [u8; 32]) -> bool {
    tag_into(salt, &[ikm], prk)
}

/// Each block is key material; the one kept for the next round is wiped
/// before this returns, on either outcome.
pub fn expand(prk: &[u8; 32], info: &[u8], out: &mut [u8]) -> bool {
    let (mut prev, mut block) = ([0u8; 32], [0u8; 32]);
    let mut have_prev = false;
    let mut done = 0usize;
    let mut counter = 1u8;
    let mut ok = true;
    while done < out.len() {
        let previous: &[u8] = if have_prev { &prev } else { &[] };
        if !tag_into(prk, &[previous, info, &[counter]], &mut block) {
            ok = false;
            break;
        }
        let n = core::cmp::min(block.len(), out.len() - done);
        out[done..done + n].copy_from_slice(&block[..n]);
        prev.copy_from_slice(&block);
        have_prev = true;
        done += n;
        counter = counter.wrapping_add(1);
    }
    wipe(&mut prev);
    wipe(&mut block);
    ok
}
