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

//! Room for a circuit whose exit takes the port a stream waits on.

use crate::circuit::{destroy, Circuit, CircuitStage, Purpose};
use crate::protocol::CIRCUIT_MAX;
use crate::trace;

use super::super::state::Manager;
use super::super::warm_rule::may_take;

/*
 * Three general circuits are the most held, and with each exit letting out
 * only the web ports a stream to another port would wait for one of them to
 * go stale, ten minutes. One that is open and carries no live stream is torn
 * down instead, with a DESTROY as retire.rs sends, and the next draw is for
 * the port. A circuit carrying a stream is never touched, whatever its exit,
 * and nor is the warm one, which is kept for installs.
 */
/// Make room for a circuit whose exit takes `port`, for a stream that is an
/// `install` or not, unless one being built already takes it.
pub(super) fn make_room(state: &mut Manager, port: u16, install: bool) {
    let general = |c: &&Circuit| c.purpose == Purpose::General;
    let coming = state.circuits.iter().filter(general).any(|c| {
        c.stage == CircuitStage::Handshaking
            && may_take(install, c.warm)
            && c.path.last().is_some_and(|exit| state.exits.takes(exit, port))
    });
    if coming || state.circuits.iter().filter(general).count() < CIRCUIT_MAX {
        return;
    }
    let idle = state.circuits.iter().filter(general).find(|c| {
        c.stage == CircuitStage::Open
            && !c.warm
            && !state.streams.iter().any(|s| s.circuit == c.id && s.stage.is_live())
    });
    let Some(id) = idle.map(|c| c.id) else { return };
    if let Some(link) = state.link.as_mut() {
        let _ = link.send(&destroy(id).encode());
    }
    state.circuits.retain(|c| c.id != id);
    trace::say_num(b"idle circuit dropped for an exit that takes a waiting stream", id as u64);
}
