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

//! Sending a kept open down a circuit, as a fresh stream there.

use crate::cell::RELAY_BEGIN;
use crate::stream::{pieces, Stream, StreamStage};
use crate::trace;

use super::super::state::Manager;
use super::send_data::send_data;
use super::send_relay::send_relay;

/// Send the open the stream at `at` holds, and the bytes it sent behind its
/// first one, on the circuit at `index`. The stream starts there afresh with
/// the same id and owner, so the caller's handle holds. False, the stream
/// left waiting, when the circuit's window would not take those bytes or the
/// BEGIN did not go.
pub(super) fn attach(state: &mut Manager, at: usize, index: usize) -> bool {
    let Some(mut held) = state.streams[at].held.take() else { return false };
    let circuit = &state.circuits[index];
    let room = circuit.last_hop().map_or(0, |hop| circuit.hops[hop].package_window.max(0));
    let id = state.streams[at].id;
    if (room as usize) < pieces(&held.early).count()
        || send_relay(state, index, RELAY_BEGIN, id, &held.body).is_err()
    {
        state.streams[at].held = Some(held);
        return false;
    }
    let (early, why) = (core::mem::take(&mut held.early), held.why);
    let mut fresh = Stream::new(id, state.circuits[index].id);
    fresh.owner = state.streams[at].owner;
    fresh.held = Some(held);
    state.streams[at] = fresh;
    /* The window was checked, so only a link that broke since fails here,
     * and the stream is ended rather than left short of its bytes. */
    if send_data(state, id, &early) != Ok(early.len()) {
        state.streams[at].stage = StreamStage::Ended(why);
        state.streams[at].held = None;
    }
    trace::say_num(b"stream begin sent", id as u64);
    true
}
