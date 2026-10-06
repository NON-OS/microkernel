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

use super::types::{ACK_IV_BYTES, ACK_PLAINTEXT_BYTES, FRAG_ID_BYTES};
use crate::crypto::aes::Ctr64Be;

/// Read back the fragment an acknowledgement of ours names.
///
/// The reverse of `ack_plaintext`: the prefix is the iv the id was encrypted
/// under. Anything that is not exactly an acknowledgement's width is not one.
/// Under a key that is not ours the id comes out as noise, which the ledger
/// simply does not hold, so nothing here has to tell the two apart.
pub fn open_ack(ack_key: &[u8; 16], payload: &[u8]) -> Option<[u8; FRAG_ID_BYTES]> {
    if payload.len() != ACK_PLAINTEXT_BYTES {
        return None;
    }
    let mut iv = [0u8; ACK_IV_BYTES];
    iv.copy_from_slice(&payload[..ACK_IV_BYTES]);
    let mut id = [0u8; FRAG_ID_BYTES];
    id.copy_from_slice(&payload[ACK_IV_BYTES..]);
    Ctr64Be::new(ack_key, &iv).apply(&mut id);
    Some(id)
}
