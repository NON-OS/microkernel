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

/* The parser's node cap. dom/ keeps its limit private, so a parsed DOM
 * of this size is taken as cut short by it. The count is read before the
 * page's scripts run, so script-built nodes never raise the notice; a page
 * of exactly this many nodes is ambiguous and is reported as cut. */
const NODE_CAP: usize = 60_000;

/* Whether the parser stopped at its node cap, for a DOM just parsed. */
pub(super) fn parser_cut(dom: &crate::browser::dom::Dom) -> bool {
    dom.nodes.len() >= NODE_CAP
}

/* A document is on screen at `url`: it becomes the current address, takes
 * its history entry, and shows in the address bar unless the reader is
 * typing there. commit_doc already noted whether the parser cut it. */
pub(super) fn committed(state: &mut State, url: &str, suppress: bool) {
    state.ui.current_url = alloc::string::String::from(url);
    record_history(state, suppress);
    state.show_url(url);
    state.mark(Change::Full);
}
