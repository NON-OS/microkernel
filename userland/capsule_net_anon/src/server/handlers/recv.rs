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

//! Collecting what has arrived on a stream.

use crate::manager::Manager;
use crate::protocol::{E_NO_STREAM, E_OK, E_RX_EMPTY, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::stream::StreamStage;

use super::closed::closed;

/// Take whatever has arrived on a stream.
///
pub fn recv(state: &mut Manager, body: &[u8], tx: &mut [u8]) -> (u16, u32) {
    if body.len() < 2 {
        return (E_NO_STREAM, 0);
    }
    let id = u16::from_le_bytes([body[0], body[1]]);
    let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) else {
        return (E_NO_STREAM, 0);
    };
    let take = stream.inbound.len().min(IPC_PAYLOAD_MAX);
    if take == 0 {
        return match stream.stage {
            StreamStage::Ended(reason) => closed(reason, tx),
            _ => (E_RX_EMPTY, 0),
        };
    }
    tx[HDR_LEN..HDR_LEN + take].copy_from_slice(&stream.inbound[..take]);
    stream.inbound.drain(..take);
    (E_OK, take as u32)
}
