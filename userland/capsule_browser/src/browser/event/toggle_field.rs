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

//! A click on a checkbox or radio button, around the page's click listeners.

use crate::browser::omnibox::Change;
use crate::browser::state::State;

use super::field_toggle::{restore, toggle, Prior};
use super::relayout::relayout;

/* As HTML orders it: the box is checked before the click listeners run,
 * so a listener reads the new state. */
pub(super) fn before_click(state: &mut State, id: usize) -> Option<Prior> {
    toggle(state.page_dom.as_mut()?, id)
}

/* After them: a listener that cancelled the click has the box put back;
 * otherwise the page hears input and change, and the box is redrawn. */
pub(super) fn after_click(state: &mut State, id: usize, prior: Prior, prevented: bool) {
    if prevented {
        if let Some(dom) = state.page_dom.as_mut() {
            restore(dom, &prior);
        }
        relayout(state);
        state.track.laid_print = None;
        state.mark(Change::Page);
    } else {
        heard_change(state, id);
    }
}

/* The reader changed control `id` (a box checked, an option chosen): the
 * page hears input then change, what its listeners asked for is acted
 * on, and the control is redrawn. */
pub(super) fn heard_change(state: &mut State, id: usize) {
    if let Some(engine) = state.engine.as_ref() {
        engine.dispatch_event(id as i32, "input");
        engine.dispatch_event(id as i32, "change");
        super::script_nav::take_script_nav(state);
    }
    relayout(state);
    state.track.laid_print = None;
    state.mark(Change::Page);
}
