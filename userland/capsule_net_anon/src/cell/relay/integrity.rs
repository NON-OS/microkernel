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

//! Lifting the four integrity bytes out of a payload and putting them back.

use super::super::geometry::{PAYLOAD_BYTES, RELAY_DIGEST_AT, RELAY_DIGEST_BYTES};

/// Take the integrity bytes out and zero the field.
///
pub fn take_digest(payload: &mut [u8; PAYLOAD_BYTES]) -> [u8; RELAY_DIGEST_BYTES] {
    let mut held = [0u8; RELAY_DIGEST_BYTES];
    let at = RELAY_DIGEST_AT..RELAY_DIGEST_AT + RELAY_DIGEST_BYTES;
    held.copy_from_slice(&payload[at.clone()]);
    payload[at].fill(0);
    held
}

/// Write the leading four bytes of `digest` into the integrity field.
///
pub fn put_digest(payload: &mut [u8; PAYLOAD_BYTES], digest: &[u8]) {
    if digest.len() < RELAY_DIGEST_BYTES {
        return;
    }
    let at = RELAY_DIGEST_AT..RELAY_DIGEST_AT + RELAY_DIGEST_BYTES;
    payload[at].copy_from_slice(&digest[..RELAY_DIGEST_BYTES]);
}
