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

//! The keyring's answers to the record operations, and its refusals as
//! the bond store names them (`protocol/errno.rs`).

use alloc::vec::Vec;

use nonos_bt_bond::KeysError;

/// OP_STORE's answer: the new record's id, and nothing else.
pub fn id(reply: &[u8]) -> Result<u32, KeysError> {
    let b: [u8; 4] = reply.try_into().map_err(|_| KeysError::Unavailable)?;
    Ok(u32::from_le_bytes(b))
}

/// OP_LIST's answer: a count, then exactly that many ids.
pub fn ids(reply: &[u8]) -> Result<Vec<u32>, KeysError> {
    let (count, rest) = reply.split_first_chunk::<4>().ok_or(KeysError::Unavailable)?;
    let count = u32::from_le_bytes(*count) as usize;
    if rest.len() / 4 != count || rest.len() % 4 != 0 {
        return Err(KeysError::Unavailable);
    }
    Ok(rest.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
}

/// The keyring's errno as the bond store reads it. Anything not named,
/// EINVAL from a keyring without L33 among them, is a keyring that cannot
/// keep bonds.
pub fn refusal(errno: i32) -> KeysError {
    match errno {
        -2 => KeysError::NotFound,
        -13 => KeysError::Denied,
        -28 => KeysError::Full,
        _ => KeysError::Unavailable,
    }
}
