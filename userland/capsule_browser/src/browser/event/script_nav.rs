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

use crate::browser::state::{Origin, State};

/* Act on what a script asked of the browser while it was running: a
 * scroll of the page (scroll_by::follow_script_scroll), a dialog to tell
 * the reader about (script_dialog), then a navigation.
 *
 * `location.assign`, `location.replace`, `location.reload` and the history
 * steps cannot take effect where they are called: the tree the script is still executing
 * against would be torn down under it. The engine parks the address
 * instead, and this collects it once the run is over.
 *
 * A page already going somewhere is left alone. The reader's own click
 * started that one, and letting a script's request overwrite it would take
 * them somewhere they did not ask to go. What the reader is typing in the
 * address bar is left alone too: the bar shows the new address at commit. */
pub fn take_script_nav(state: &mut State) {
    /* A scrollTo, scrollBy or scrollIntoView moves the window first: its
     * scroll listeners may themselves ask for a navigation. */
    super::scroll_by::follow_script_scroll(state);
    /* An alert, confirm or prompt is said to the reader, navigation or
     * not: a page that alerts "Saved" and moves on still said it. */
    super::script_dialog::take_script_dialog(state);
    if state.pending_nav.is_some() {
        return;
    }
    let Some(engine) = state.engine.as_ref() else {
        return;
    };
    match (engine.take_navigation(), engine.take_history_step()) {
        (Some(next), _) if !next.is_empty() => {
            state.pending_nav = Some(next);
            state.ui.origin = Origin::Auto;
        }
        /* history.back(), forward() or go(n): the same step the toolbar's
         * buttons take, and nothing when it would run off either end. */
        (_, Some(delta)) => {
            super::nav_history::nav_history(state, delta);
        }
        _ => {}
    }
}
