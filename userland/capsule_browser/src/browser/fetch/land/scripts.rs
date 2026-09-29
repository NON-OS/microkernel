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

//! External scripts run in document order, whenever they arrived.

use crate::browser::state::State;

/*
 * A script runs after the stylesheets before it have applied, as it did when
 * everything was fetched in turn: page code that measures the layout must
 * see it styled. The fetches still run side by side; only running waits.
 */
/// Run each held script whose turn has come. A framework bundle builds its
/// DOM here, which is the render, so the page is laid out again after.
pub(in crate::browser::fetch) fn run_held(state: &mut State) -> bool {
    let sheets = !state.css_queue.is_empty() || state.pool.live.iter().any(|f| f.css);
    if sheets {
        return false;
    }
    let mut ran = false;
    while let Some(at) = state.pool.held.iter().position(|(o, _)| *o == state.pool.run_order) {
        let (_, body) = state.pool.held.swap_remove(at);
        state.pool.run_order += 1;
        if body.is_empty() {
            continue;
        }
        if let Some(engine) = state.engine.as_ref() {
            let _ = engine.eval(&alloc::string::String::from_utf8_lossy(&body));
        }
        ran = true;
    }
    if ran {
        crate::browser::event::relayout(state);
    }
    ran
}
