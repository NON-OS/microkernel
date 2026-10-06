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

use crate::browser::dom::Dom;
use crate::browser::event::dom_print::dom_print;
use crate::browser::state::State;

/* The fingerprint the kept styles are checked against: the document and
 * its fetched CSS, with and without form values, and the focused field
 * when a selector tests :focus, since those styles follow it. */
pub(super) fn print(state: &State, dom: &Dom) -> (u64, u64) {
    let print = dom_print(dom, &state.page_css);
    if !state.css_cache.as_ref().is_some_and(|c| c.state_mask() & 2 != 0) {
        return print;
    }
    let f = state.focus.map_or(0, |f| (f as u64).wrapping_add(1).wrapping_mul(0x9e37_79b9));
    (print.0 ^ f, print.1 ^ f)
}
