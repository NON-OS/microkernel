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

//! A finished script request, handed to the callback that asked for it.

use alloc::string::String;

use crate::browser::http;
use crate::browser::state::State;

/// Call the page back with the status and body; a request that failed still
/// calls back, with status 0 and nothing, so page code can branch on ok.
pub(in crate::browser::fetch) fn land_js(state: &mut State, raw: Option<&[u8]>) -> bool {
    let (status, body) = raw
        .and_then(http::response::parse)
        .map(|r| (r.status, String::from_utf8_lossy(&r.body).into_owned()))
        .unwrap_or((0, String::new()));
    let (Some(dom), Some(world)) = (state.page_dom.as_mut(), state.world.as_mut()) else {
        return false;
    };
    let Some(cb) = world.net_active.take() else { return false };
    let dirty = crate::browser::js::deliver_net(dom, world, cb, status, body);
    if dirty {
        crate::browser::event::relayout(state);
    }
    dirty
}
