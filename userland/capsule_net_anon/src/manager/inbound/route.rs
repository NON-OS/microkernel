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

//! Routing one opened relay message by its command.

use crate::cell::{body, unpack, PAYLOAD_BYTES};
use crate::cell::{RELAY_CONNECTED, RELAY_DATA, RELAY_END, RELAY_SENDME, RELAY_TRUNCATED};
use crate::circuit::CircuitStage;
use crate::trace;

use super::super::state::Manager;
use super::data::data;
use super::status::{connected, ended, granted};

pub(crate) fn deliver(
    state: &mut Manager,
    index: usize,
    hop: usize,
    payload: &[u8; PAYLOAD_BYTES],
) {
    let header = unpack(payload);
    let Some(body) = body(payload) else { return };
    match header.command {
        RELAY_DATA => data(state, index, header.stream, body),
        RELAY_CONNECTED => connected(state, header.stream, body),
        RELAY_END => ended(state, header.stream, body),
        RELAY_SENDME => granted(state, index, hop),
        RELAY_TRUNCATED => truncated(state, index, body),
        _ => {}
    }
}

/*
 * Not ignorable, unlike padding. TRUNCATED says the circuit is now shorter than
 * this side believes: a hop beyond the sender is gone, so the exit this circuit
 * was built to is no longer on the end of it. Dropping the message would leave
 * the circuit looking Open while every cell addressed to the exit went nowhere,
 * and every stream on it would hang until its deadline rather than fail and be
 * retried on another path.
 */
fn truncated(state: &mut Manager, index: usize, body: &[u8]) {
    let why = body.first().copied().unwrap_or(0);
    trace::say_two(b"circuit truncated", state.circuits[index].id as u64, why as u64);
    state.circuits[index].stage = CircuitStage::Dead;
}
