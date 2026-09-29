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

//! One tick of the fetch machine.

use super::constants::TICK_MS;
use super::net_wire::NetWire;
use super::pool::Idle;
use crate::browser::net::Source;
use crate::browser::state::State;

/*
 * Every fetch in flight is stepped, the navigation first; each one that
 * ended lands in this call, its connection kept or closed; and the room that
 * frees is filled from the queues, also in this call. It used to be one
 * fetch per tick, finished on the tick after it ended, and a repaint every
 * tick whether or not anything had moved: now the window is drawn again only
 * when the status line changed or something landed on the page.
 */
/// Advance the fetches; true when something visible changed.
pub fn tick(state: &mut State) -> bool {
    if state.sockets_port == 0 {
        return false;
    }
    let status = state.status.clone();
    let mut w = NetWire(state.sockets_port);
    let until = w.now_ms().saturating_add(TICK_MS);
    let mut shown = super::nav::step(state, &mut w, until);
    for job in state.pool.step(&mut w, until) {
        shown |= super::land::land(state, &mut w, job);
    }
    if let Some(kept) = state.keep.take() {
        let idle = Idle::kept(kept, w.now_ms());
        state.pool.park(&mut w, idle);
    }
    shown |= super::launch::launch(state, &mut w);
    shown || state.status != status
}
