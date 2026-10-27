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

//! A stream that waits for a circuit whose exit takes its port.

use crate::path::{any_exit, Relay};
use crate::stream::{begin_body, Held, Stream};
use crate::trace;

use super::super::state::Manager;
use super::free_id::free_id;
use super::pick_circuit::usable;
use super::send_relay::SendError;

/// Open a stream to `port` that no open circuit's exit takes. It waits with
/// no circuit, as a stream to an onion service waits for its lookup, while a
/// circuit whose exit takes the port is built. With no circuit this stream
/// may take at all, or no exit for the port, it is refused as before.
pub(super) fn wait(
    state: &mut Manager,
    host: &[u8],
    port: u16,
    now: u64,
    owner: u32,
    install: bool,
) -> Result<u16, SendError> {
    let open = (0..state.circuits.len()).any(|index| usable(state, index, now, install));
    let takes = |exit: &Relay| state.exits.takes(exit, port);
    if !open || !any_exit(&state.relays, &state.weights, takes) {
        return Err(SendError::NoCircuit);
    }
    let body = begin_body(host, port).ok_or(SendError::TooLong)?;
    let id = free_id(state).ok_or(SendError::TableFull)?;
    state.next_stream = id;
    let mut stream = Stream::new(id, 0);
    stream.owner = owner;
    stream.held = Some(Held::new(body, port, now, install));
    state.streams.push(stream);
    trace::say_num(b"stream waits for an exit that takes its port", id as u64);
    Ok(id)
}
