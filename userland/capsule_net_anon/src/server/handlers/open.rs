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

//! Opening a stream on behalf of a caller.

use crate::manager::{open_stream, Bootstrap, Manager, SendError};
use crate::protocol::{
    E_BAD_LEN, E_DIRECTORY_STALE, E_NO_CIRCUIT, E_NO_DIRECTORY, E_NO_LINK, E_NO_PATH, E_OK,
    E_TABLE_FULL, HDR_LEN,
};

/// Request body: port as two bytes, then the host.
///
pub fn open(state: &mut Manager, body: &[u8], now: u64, tx: &mut [u8]) -> (u16, u32) {
    if body.len() < 3 {
        return (E_BAD_LEN, 0);
    }
    if let Some(why) = not_ready(state, now) {
        return (why, 0);
    }
    let port = u16::from_le_bytes([body[0], body[1]]);
    match open_stream(state, &body[2..], port, now) {
        Ok(id) => {
            tx[HDR_LEN..HDR_LEN + 2].copy_from_slice(&id.to_le_bytes());
            (E_OK, 2)
        }
        Err(SendError::TableFull) => (E_TABLE_FULL, 0),
        Err(SendError::TooLong) => (E_BAD_LEN, 0),
        Err(SendError::NoLink) => (E_NO_LINK, 0),
        Err(_) => (E_NO_CIRCUIT, 0),
    }
}

/*
 * A caller that cannot open a stream deserves to know which of four different
 * situations it is in. Answering one blanket error for all of them is how a front
 * end ends up retrying a transport that is still bootstrapping and giving up on
 * one that merely has no circuit yet.
 */
fn not_ready(state: &Manager, now: u64) -> Option<u16> {
    if state.bootstrap != Bootstrap::Ready {
        return Some(E_NO_DIRECTORY);
    }
    if now >= state.valid_until {
        return Some(E_DIRECTORY_STALE);
    }
    if state.relays.is_empty() {
        return Some(E_NO_PATH);
    }
    if state.link.is_none() {
        return Some(E_NO_LINK);
    }
    None
}
