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

//! Stopping everything the page has in flight.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::wire::Wire;
use crate::browser::state::State;

/*
 * Stop, Escape and Home come here: the navigation, the pool's fetches and
 * its kept connections, and every queue. A page held blank for its
 * stylesheets is laid out with what it has, rather than left waiting for
 * sheets that are no longer coming.
 */
/// Close every connection the browser holds and clear what waits to start.
pub fn cancel_all(state: &mut State) {
    let mut w = NetWire(state.sockets_port);
    if let Some(f) = state.fetch.take() {
        w.close(f.handle);
    }
    let sheets = !state.css_queue.is_empty() || state.pool.live.iter().any(|f| f.css);
    state.pool.cancel_all(&mut w);
    if let Some(kept) = state.keep.take() {
        w.close(kept.handle);
    }
    state.css_queue.clear();
    state.font_queue.clear();
    state.script_queue.clear();
    state.image_queue.clear();
    if let Some(world) = state.world.as_mut() {
        world.net.clear();
        world.net_active = None;
    }
    if sheets {
        crate::browser::event::relayout(state);
    }
}
