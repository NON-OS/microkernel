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

//! Starting the page's next script request.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Refused;
use crate::browser::state::State;
use crate::browser::url;

/*
 * One script request runs at a time: the engine holds a single callback for
 * the answer. A request that cannot even start is answered at once with
 * status 0, as a failed one is, so page code waiting on it can go on.
 */
/// Start the next script request if none is running; true if the page
/// changed because one was answered on the spot.
pub(in crate::browser::fetch) fn launch_js(state: &mut State, w: &mut NetWire) -> bool {
    let Some(world) = state.world.as_ref() else { return false };
    let Some((target, _)) = world.net.first().filter(|_| world.net_active.is_none()) else {
        return false;
    };
    let abs = match state.base.as_ref() {
        Some(b) => url::join(b, target),
        None => target.clone(),
    };
    let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
    let started = match url::parse(&abs) {
        Some(u) => state.pool.start(w, u, proxy).map(|f| f.js_req = true),
        None => Err(Refused::Failed("bad url")),
    };
    if started == Err(Refused::Busy) {
        return false;
    }
    let Some(world) = state.world.as_mut() else { return false };
    let (_, cb) = world.net.remove(0);
    world.net_active = Some(cb);
    started.is_err() && crate::browser::fetch::land::land_js(state, None)
}
