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

use super::super::types::CryptoError;
use super::super::wipe::wipe;
use super::hmac::hmac_blake3;
use alloc::vec::Vec;

/// HKDF-Expand. The counter starts at one and each round feeds back the
/// previous block, so output longer than one hash is still bound to the key.
pub fn expand(prk: &[u8; 32], info: &[u8], out: &mut [u8]) -> Result<(), CryptoError> {
    if out.len() > 255 * 32 {
        return Err(CryptoError::Kdf);
    }
    // Each block is key material: the one kept for the next round and every
    // message built from it are wiped, on either outcome.
    let mut previous = [0u8; 32];
    let mut have_previous = false;
    let mut done = 0usize;
    let mut counter: u8 = 1;
    let mut result = Ok(());
    while done < out.len() {
        let kept = if have_previous { &previous[..] } else { &[][..] };
        let mut message = Vec::with_capacity(kept.len() + info.len() + 1);
        message.extend_from_slice(kept);
        message.extend_from_slice(info);
        message.push(counter);
        result = hmac_blake3(prk, &message, &mut previous);
        wipe(&mut message);
        if result.is_err() {
            break;
        }
        have_previous = true;
        let take = core::cmp::min(32, out.len() - done);
        out[done..done + take].copy_from_slice(&previous[..take]);
        done += take;
        counter += 1;
    }
    wipe(&mut previous);
    result
}
