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

//! Handing bytes from a caller to a stream.

use crate::manager::{send_data, Manager, SendError};
use crate::protocol::{E_NO_CIRCUIT, E_NO_LINK, E_NO_STREAM, E_OK, E_WOULD_BLOCK, HDR_LEN};

use super::closed::closed;

/// Request body: stream id as two bytes, then the bytes to send.
///
pub fn send(state: &mut Manager, body: &[u8], tx: &mut [u8]) -> (u16, u32) {
    if body.len() < 2 {
        return (E_NO_STREAM, 0);
    }
    let id = u16::from_le_bytes([body[0], body[1]]);
    match send_data(state, id, &body[2..]) {
        Ok(sent) => {
            tx[HDR_LEN..HDR_LEN + 4].copy_from_slice(&(sent as u32).to_le_bytes());
            (E_OK, 4)
        }
        Err(SendError::WouldBlock) => (E_WOULD_BLOCK, 0),
        Err(SendError::Ended(reason)) => closed(reason, tx),
        Err(SendError::NoLink) => (E_NO_LINK, 0),
        Err(SendError::NoCircuit) => (E_NO_CIRCUIT, 0),
        Err(_) => (E_NO_STREAM, 0),
    }
}
