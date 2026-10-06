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

use super::record_history::record_history;

/* Whether the parser left part of the page out, for a DOM just parsed and
 * before its scripts run, so script-built nodes never raise the notice.
 * This used to guess from the node count alone, so a page whose links or
 * classes were dropped at the attribute limits showed no notice at all. */
pub(super) fn parser_cut(dom: &crate::browser::dom::Dom) -> bool {
    dom.cut_short()
}

/* A document is on screen at `url`: it becomes the current address, takes
 * its history entry, and shows in the address bar unless the reader is
 * typing there. It comes before commit_doc homes the document and runs
 * its scripts, so they read this page's place in history. */
pub(super) fn committed(state: &mut State, url: &str, suppress: bool) {
    state.ui.current_url = alloc::string::String::from(url);
    record_history(state, suppress);
    state.show_url(url);
    state.mark(Change::Full);
}
