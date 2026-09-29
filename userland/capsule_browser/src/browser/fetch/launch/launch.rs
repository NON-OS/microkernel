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

//! Filling the pool from the page's queues, most urgent first.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::wire::Wire;
use crate::browser::state::State;

/*
 * Stylesheets go first, being render-blocking, then fonts, then scripts,
 * then images and script requests turn about for the rest. Each starts in
 * this call; one whose host is at its limit waits without holding back the
 * others. The mixnet carries one conversation, so nothing starts beside the
 * navigation there.
 */
/// Start what the pool has room for; true if the page changed doing so.
pub(in crate::browser::fetch) fn launch(state: &mut State, w: &mut NetWire) -> bool {
    if state.sockets_port == 0 || (w.mixnet() && state.fetch.is_some()) {
        return false;
    }
    let mut shown = super::launch_css::css(state, w);
    super::launch_css::fonts(state, w);
    shown |= super::launch_scripts::launch(state, w);
    state.img_turn = !state.img_turn;
    if state.img_turn {
        shown |= crate::browser::image::pump(state, w);
        shown |= super::launch_js::launch_js(state, w);
    } else {
        shown |= super::launch_js::launch_js(state, w);
        shown |= crate::browser::image::pump(state, w);
    }
    shown
}
