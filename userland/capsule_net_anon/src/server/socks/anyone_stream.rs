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

//! Reading from and ending an Anyone stream a SOCKS caller holds.

extern crate alloc;

use alloc::vec::Vec;

use crate::manager::{send_end, Manager};
use crate::stream::REASON_DONE;

/// Take up to `max` bytes that have arrived on stream `id`.
pub fn take(state: &mut Manager, id: u16, max: usize) -> Vec<u8> {
    let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) else {
        return Vec::new();
    };
    let n = stream.inbound.len().min(max);
    stream.inbound.drain(..n).collect()
}

/*
 * END goes only on a stream the exit still holds open. One it already ended
 * needs nothing sent, and an END for it would be a cell no other client sends.
 */
pub fn close(state: &mut Manager, id: u16) {
    let Some(stream) = state.streams.iter().find(|s| s.id == id) else {
        return;
    };
    if stream.stage.is_live() {
        let circuit = stream.circuit;
        if let Some(index) = state.circuits.iter().position(|c| c.id == circuit) {
            let _ = send_end(state, index, id, REASON_DONE);
        }
    }
    state.streams.retain(|s| s.id != id);
}
