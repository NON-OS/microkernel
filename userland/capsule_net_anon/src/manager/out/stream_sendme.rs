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

//! Paying the SENDMEs each stream owes.

extern crate alloc;

use alloc::vec::Vec;

use crate::cell::RELAY_SENDME;
use crate::protocol::STREAM_HIGH_WATER;

use super::super::state::Manager;
use super::send_relay::send_relay;

/*
 * A stream SENDME carries no digest. tor-spec authenticates the circuit level
 * acknowledgement, not the stream level one, so the body is empty and the stream
 * id in the header is what says which stream is being granted room.
 *
 * A stream whose buffer is above the high water mark is passed over. It still
 * owes the grant and will send it on a later turn; until then the far end has no
 * room and stops, which is the only way this side can tell it to slow down.
 */
pub(super) fn stream_tick(state: &mut Manager) {
    let mut due: Vec<(u16, u32)> = Vec::new();
    for stream in state.streams.iter_mut() {
        if stream.take_sendme_due(STREAM_HIGH_WATER) {
            due.push((stream.id, stream.circuit));
        }
    }
    for (id, circuit_id) in due {
        let Some(index) = state.circuits.iter().position(|c| c.id == circuit_id) else {
            continue;
        };
        let _ = send_relay(state, index, RELAY_SENDME, id, &[]);
    }
}
