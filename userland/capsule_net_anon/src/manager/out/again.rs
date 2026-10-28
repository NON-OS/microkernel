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

//! Sending an open once more, to another exit, when the first exit ended it
//! before it connected.

use crate::path::{any_exit, Relay};
use crate::stream::{retries, sends_again, Held, StreamStage, WAIT_SECONDS};
use crate::trace;

use super::super::state::Manager;
use super::attach::attach;
use super::pick_circuit::pick;

/*
 * Before this, an exit that refused the port ended the stream and the caller
 * was told, and the next stream to that port was as likely to meet the same
 * exit. Now the exit is remembered as refusing the port, so no draw for it
 * picks that exit again this boot, and the open goes once more: at once on a
 * circuit whose exit takes the port, or once circuit_tick has built one. The
 * caller sees one stream throughout and hears only the second END, if any.
 */
/// The stream `id` on the circuit at `index` ended with `why` before it was
/// connected. Remember the exit refused its port, and send its open to
/// another exit if the rule says so; otherwise leave it ended, and let go of
/// the open it held.
pub(in crate::manager) fn again(state: &mut Manager, index: usize, id: u16, why: u8, now: u64) {
    let circuit = state.circuits[index].id;
    let Some(at) = state.streams.iter().position(|s| s.circuit == circuit && s.id == id) else {
        return;
    };
    let Some(held) = state.streams[at].held.take() else { return };
    let (port, install) = (held.port, held.install);
    if let Some(exit) = state.circuits[index].path.last().filter(|_| retries(why)) {
        state.exits.refused.note(exit.rsa_identity, port);
    }
    let takes = |exit: &Relay| state.exits.takes(exit, port);
    if !sends_again(Some(&held), why) || !any_exit(&state.relays, &state.weights, takes) {
        return;
    }
    let stream = &mut state.streams[at];
    let until = now.saturating_add(WAIT_SECONDS);
    stream.held = Some(Held { again: true, why, until, ..held });
    stream.stage = StreamStage::Opening;
    stream.circuit = 0;
    trace::say_num(b"stream refused before it connected, sent to another exit", id as u64);
    if let Some(next) = pick(state, now, port, install) {
        attach(state, at, next);
    }
}
