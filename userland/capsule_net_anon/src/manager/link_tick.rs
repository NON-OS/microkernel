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

//! Keeping one link to one guard open.

use crate::link::open;
use crate::link::LinkError;
use crate::trace;

use super::guard::{draw, Guard};
use super::link_stage::stage_of;
use super::state::Manager;

// One link, to one guard, reused by every circuit. A link per circuit would
// show the guard a new TLS session for each.
pub fn tick(state: &mut Manager, now: u64) {
    if state.link.is_some() || !state.usable_at(now) {
        return;
    }
    if state.guard.as_ref().is_some_and(Guard::exhausted) {
        trace::say(b"guard unreachable three times, choosing another");
        state.guard = None;
    }
    if state.guard.is_none() {
        state.guard = draw(state);
    }
    let Some(guard) = state.guard.as_mut() else { return };
    if !guard.due(now) {
        return;
    }
    guard.tried_at = now;
    let relay = guard.relay.clone();
    match open(state.tcp_port, &relay, now) {
        Ok(link) => {
            trace::say_addr(b"link open to guard", relay.address, relay.or_port);
            state.link = Some(link);
            if let Some(g) = state.guard.as_mut() {
                g.failures = 0;
            }
        }
        Err(cause) => {
            if let Some(g) = state.guard.as_mut() {
                g.failures = g.failures.saturating_add(1);
            }
            // Which stage gave up, not just that one did. A TLS failure is this client's,
            // a version mismatch is a fork in the protocol.
            trace::say_addr(stage_of(cause), relay.address, relay.or_port);
        }
    }
}
