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

//! Handing bytes to an open stream.

use crate::cell::RELAY_DATA;
use crate::stream::{pieces, StreamStage};

use super::super::state::Manager;
use super::send_relay::{send_relay, SendError};
use super::window::{room, spend};

/// Send `data` on stream `id` and report how many bytes went.
///
pub fn send_data(state: &mut Manager, id: u16, data: &[u8]) -> Result<usize, SendError> {
    let stream = state.streams.iter().find(|s| s.id == id).ok_or(SendError::NoStream)?;
    if let StreamStage::Ended(reason) = stream.stage {
        return Err(SendError::Ended(reason));
    }
    let circuit_id = stream.circuit;
    let index =
        state.circuits.iter().position(|c| c.id == circuit_id).ok_or(SendError::NoCircuit)?;

    let mut sent = 0usize;
    for piece in pieces(data) {
        if !room(state, index, id) {
            break;
        }
        send_relay(state, index, RELAY_DATA, id, piece)?;
        spend(state, index, id);
        sent += piece.len();
    }
    if sent == 0 && !data.is_empty() {
        return Err(SendError::WouldBlock);
    }
    Ok(sent)
}
