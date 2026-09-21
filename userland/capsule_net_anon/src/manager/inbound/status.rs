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

//! The messages that change a stream or circuit state.

use crate::circuit::window::CIRCUIT_INCREMENT;
use crate::stream::{connected_is_valid, needs_another_exit, reason, StreamStage};
use crate::trace;

use super::super::state::Manager;
use super::blame::blame;

const REASON_MISC: u8 = 1;

pub(super) fn connected(state: &mut Manager, id: u16, body: &[u8]) {
    let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) else {
        return;
    };
    if !connected_is_valid(body) {
        stream.stage = StreamStage::Ended(REASON_MISC);
        return;
    }
    stream.stage = StreamStage::Open;
    trace::say_num(b"stream connected", id as u64);
}

pub(super) fn ended(state: &mut Manager, id: u16, body: &[u8]) {
    let why = reason(body);
    let mut on_circuit = None;
    if let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) {
        stream.stage = StreamStage::Ended(why);
        on_circuit = Some(stream.circuit);
    }
    trace::say_two(b"stream ended", id as u64, why as u64);
    if needs_another_exit(why) {
        blame(state, on_circuit);
    }
}

pub(super) fn granted(state: &mut Manager, index: usize, hop: usize) {
    if let Some(hop) = state.circuits[index].hops.get_mut(hop) {
        hop.package_window = hop.package_window.saturating_add(CIRCUIT_INCREMENT);
    }
}
