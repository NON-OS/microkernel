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
use crate::stream::{begin_body, may_open, Held, Stream};
use crate::trace;

use super::super::state::Manager;
use super::free_id::free_id;
use super::pick_circuit::pick;
use super::send_relay::{send_relay, SendError};
use super::wait::wait;

/// Open a stream to `host` on `port` through an open circuit whose exit takes
/// the port; an `install` takes the warm circuit when one is open
/// (pick_circuit.rs). With none, the stream may wait for one (wait.rs).
pub fn open_stream(
    state: &mut Manager,
    host: &[u8],
    port: u16,
    now: u64,
    owner: u32,
    install: bool,
) -> Result<u16, SendError> {
    if !may_open(&state.streams, owner, STREAM_MAX) {
        return Err(SendError::TableFull);
    }
    /* A .anyone name is an onion service, never a host name for an exit:
     * sent in a BEGIN it would tell the exit which service is being reached. */
    if crate::onion::address::is_onion(host) {
        return super::super::onion::open(state, host, port, now, owner);
    }
    let Some(index) = pick(state, now, port, install) else {
        return wait(state, host, port, now, owner, install);
    };
    let body = begin_body(host, port).ok_or(SendError::TooLong)?;
    let circuit = state.circuits[index].id;
    let id = free_id(state).ok_or(SendError::TableFull)?;
    state.next_stream = id;

    send_relay(state, index, RELAY_BEGIN, id, &body)?;
    let mut stream = Stream::new(id, circuit);
    stream.owner = owner;
    stream.held = Some(Held::new(body, port, now, install));
    state.streams.push(stream);
    trace::say_num(b"stream begin sent", id as u64);
    Ok(id)
}
