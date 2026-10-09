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

//! A page's scripts run in document order, whenever they arrived.

use crate::browser::js::next_ready;
use crate::browser::state::State;

/*
 * A script runs after the stylesheets before it have applied, as it did when
 * everything was fetched in turn: page code that measures the layout must
 * see it styled. The fetches still run side by side; only running waits.
 * Inline scripts hold their place in the same order as external ones, so
 * the inline code that calls a library runs after the library.
 */
/// Run each held script whose turn has come, then the page's load events
/// once the last has run. A framework bundle builds its DOM here, which is
/// the render, so the page is laid out again after.
pub(in crate::browser::fetch) fn run_held(state: &mut State) -> bool {
    let sheets = !state.css_queue.is_empty() || state.pool.live.iter().any(|f| f.css);
    if sheets && !state.style_hold.over {
        return false;
    }
    let mut ran = false;
    if let Some(engine) = state.engine.as_ref() {
        engine.set_budget(crate::browser::qjs_run::PAGE_SCRIPT_BUDGET_MS);
    }
    while let Some(at) = next_ready(&state.pool.held, state.pool.run_order) {
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
    ran |= loaded(state);
    if let Some(engine) = state.engine.as_ref() {
        engine.set_budget(crate::browser::qjs_run::SCRIPT_BUDGET_MS);
    }
    /* Laid out first, so a scrollTo the scripts asked for is held to the
     * page they built. */
    if ran {
        crate::browser::event::relayout(state);
        crate::browser::event::take_script_nav(state);
    }
    ran
}

/*
 * readyState goes to "interactive" and DOMContentLoaded fires once every
 * script in the plan has run; then "complete" and load. Images do not hold
 * load back here: they decode as they arrive and relayout on their own,
 * and a page that waited on them would sit at "interactive" behind the
 * slowest one, which over a mixnet is a long time for code that only
 * wanted the tree.
 */
fn loaded(state: &mut State) -> bool {
    let Some(total) = state.pool.scripts else { return false };
    if state.pool.run_order < total {
        return false;
    }
    state.pool.scripts = None;
    let Some(engine) = state.engine.as_ref() else { return false };
    let interactive = engine.advance_ready_state(false);
    let complete = engine.advance_ready_state(true);
    interactive || complete
}
