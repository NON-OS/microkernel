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


//! Taking a client authorization key from a caller, or taking it back.

use crate::manager::{forget_client_key, set_client_key, KeyError, Manager};
use crate::protocol::{E_BAD_LEN, E_KEY_HELD, E_OK, E_TABLE_FULL};

/// Request body: a key line, `<address>:descriptor:x25519:<base32 secret>`,
/// which the caller read from the keyring or a file the user holds; or a
/// bare address, to take back the key this caller gave for it. The reply
/// carries no body, so nothing of the key comes back out.
pub fn client_auth(state: &mut Manager, sender: u32, body: &[u8]) -> u16 {
    if body.is_empty() || body.len() > 256 {
        return E_BAD_LEN;
    }
    let result = if body.contains(&b':') {
        set_client_key(state, body, sender)
    } else {
        forget_client_key(state, body, sender)
    };
    match result {
        Ok(()) => E_OK,
        Err(KeyError::Malformed) => E_BAD_LEN,
        Err(KeyError::NotYours) => E_KEY_HELD,
        Err(KeyError::Full) => E_TABLE_FULL,
    }
}
