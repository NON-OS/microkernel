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

use crate::browser::js;
use crate::browser::state::State;

use super::relayout::relayout;
use super::script_nav::take_script_nav;

/* One app tick for the page's timers. Returns whether the screen needs
 * repainting: a timer changed the display list, or asked to navigate.
 * Timers that ran and changed nothing (an animation loop, a poll) cost a
 * fingerprint of the document and report false, so the tick goes on to
 * the fetch pumps instead of stalling the load behind them. */
pub fn js_tick(state: &mut State) -> bool {
    /* The page's own timers, in the engine that ran its scripts. */
    let ran = match state.engine.as_ref() {
        Some(engine) => engine.flush_timers(nonos_libc::mk_uptime_ms() as u64) > 0,
        None => false,
    };
    let mut changed = false;
    if ran {
        changed = relayout(state);
        take_script_nav(state);
    }
    let dirty = match (state.page_dom.as_mut(), state.world.as_mut()) {
        (Some(dom), Some(world)) => js::pump_timers(dom, world).1,
        _ => false,
    };
    if dirty {
        changed |= relayout(state);
    }
    changed || state.pending_nav.is_some()
}
