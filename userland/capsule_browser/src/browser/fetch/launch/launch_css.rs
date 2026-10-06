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

//! Starting stylesheet and font fetches.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Refused;
use crate::browser::state::State;
use crate::browser::url;

/// A sheet that cannot even start still counts as tried, or a page whose
/// only stylesheet host is down would stay blank waiting for it.
pub(in crate::browser::fetch) fn css(state: &mut State, w: &mut NetWire) -> bool {
    let (mut shown, mut at) = (false, 0);
    while at < state.css_queue.len() {
        let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
        let started = match url::parse(&state.css_queue[at]) {
            Some(u) => state.pool.start(w, u, proxy).map(|f| f.css = true),
            None => Err(Refused::Failed("bad url")),
        };
        if started == Err(Refused::Busy) {
            at += 1;
            continue;
        }
        state.css_queue.remove(at);
        if started.is_err() {
            crate::browser::fetch::apply_css::apply_css(state, None, None);
            shown = true;
        }
    }
    shown
}

/// A face that cannot start is skipped; its text keeps the built-in face.
pub(in crate::browser::fetch) fn fonts(state: &mut State, w: &mut NetWire) {
    let mut at = 0;
    while at < state.font_queue.len() {
        let (key, target) = state.font_queue[at].clone();
        let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
        let started = match url::parse(&target) {
            Some(u) => state.pool.start(w, u, proxy).map(|f| f.font = key),
            None => Err(Refused::Failed("bad url")),
        };
        if started == Err(Refused::Busy) {
            at += 1;
            continue;
        }
        state.font_queue.remove(at);
    }
}
