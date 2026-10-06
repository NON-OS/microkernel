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

//! External scripts, fetched side by side.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Refused;
use crate::browser::state::State;
use crate::browser::url;

/// Start every queued script there is room for. Each carries its place in
/// the page's script order, given when the page was planned; one whose URL
/// cannot be fetched takes its place with nothing to run, so the scripts
/// after it are not held up.
pub(in crate::browser::fetch) fn launch(state: &mut State, w: &mut NetWire) -> bool {
    let mut ran = false;
    while let Some((order, target)) = state.script_queue.first().cloned() {
        let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
        let started = match url::parse(&target) {
            Some(u) => state.pool.start(w, u, proxy).map(|f| {
                f.script = true;
                f.order = order;
            }),
            None => Err(Refused::Failed("bad url")),
        };
        match started {
            Err(Refused::Busy) => break,
            Err(Refused::Failed(_)) => state.pool.held.push((order, alloc::vec::Vec::new())),
            Ok(()) => {}
        }
        state.script_queue.remove(0);
        ran |= crate::browser::fetch::land::run_held(state);
    }
    ran
}
