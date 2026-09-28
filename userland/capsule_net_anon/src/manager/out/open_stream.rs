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

//! Asking the exit to open a connection.

use crate::cell::RELAY_BEGIN;
use crate::protocol::STREAM_MAX;
use crate::stream::{begin_body, next_id, Stream};
use crate::trace;

use super::super::state::Manager;
use super::pick_circuit::pick;
use super::send_relay::{send_relay, SendError};

/// Open a stream to `host` on `port` through an open circuit.
///
pub fn open_stream(
    state: &mut Manager,
    host: &[u8],
    port: u16,
    now: u64,
) -> Result<u16, SendError> {
    if state.streams.len() >= STREAM_MAX {
        return Err(SendError::TableFull);
    }
    let index = pick(state, now).ok_or(SendError::NoCircuit)?;
    let body = begin_body(host, port).ok_or(SendError::TooLong)?;
    let circuit = state.circuits[index].id;
    let id = free_id(state).ok_or(SendError::TableFull)?;
    state.next_stream = id;

    send_relay(state, index, RELAY_BEGIN, id, &body)?;
    state.streams.push(Stream::new(id, circuit));
    trace::say_num(b"stream begin sent", id as u64);
    Ok(id)
}

/*
 * Ids are allocated unique across every open stream, not only within the
 * circuit, because the id is also the handle a caller holds: two streams
 * sharing one would each receive the other's payload through the server.
 * Relay cells are still matched by (circuit, id). The counter running on past sixty five thousand is fine
 * until the wrapped value lands on an id still open on the same circuit. Reaching
 * that needs more streams in one circuit's life than it will ever see, but the
 * consequence if it ever happened is two streams sharing an id and each receiving
 * the other's payload, which is not a thing to leave to arithmetic.
 */

fn free_id(state: &Manager) -> Option<u16> {
    let mut id = state.next_stream;
    for _ in 0..u16::MAX {
        id = next_id(id);
        if !state.streams.iter().any(|s| s.id == id) {
            return Some(id);
        }
    }
    None
}
