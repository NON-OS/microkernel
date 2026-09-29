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

use crate::browser::omnibox::Change;
use crate::browser::state::State;

use super::relayout::relayout;

/* What the page's listeners made of a click. */
pub(super) struct ClickResult {
    /* Some listener ran; it may have changed the DOM. */
    pub fired: bool,
    /* A listener called preventDefault: the click's own action is off. */
    pub prevented: bool,
}

/* Dispatch a click on a DOM node to the page engine's listeners. A listener
 * may mutate the DOM through the engine's node bindings, so the page relays
 * out when one fires. A navigation a handler asked for could not be acted
 * on while the script held the tree, so it is collected here. */
pub(super) fn js_click(state: &mut State, node: usize) -> ClickResult {
    let (fired, prevented) = match state.engine.as_ref() {
        Some(engine) => {
            let fired = engine.dispatch_event(node as i32, "click") > 0;
            (fired, fired && engine.default_prevented())
        }
        None => return ClickResult { fired: false, prevented: false },
    };
    if fired {
        relayout(state);
        state.track.laid_print = None;
        state.mark(Change::Page);
    }
    super::script_nav::take_script_nav(state);
    ClickResult { fired, prevented }
}
