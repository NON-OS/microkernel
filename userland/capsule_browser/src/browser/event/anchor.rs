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

use crate::browser::omnibox::ScrollAct;
use crate::browser::state::State;

/* Scroll to the element a fragment names: the element with that id, else
 * an <a> with that name. An empty fragment or "top" means the top of the
 * page. Returns false, leaving the scroll alone, when nothing matches. */
pub(super) fn scroll_to_fragment(state: &mut State, frag: &str) -> bool {
    let y = if frag.is_empty() || frag.eq_ignore_ascii_case("top") {
        Some(0)
    } else {
        state.page_dom.as_ref().and_then(|dom| {
            let by_id = dom.nodes.iter().position(|n| n.attr("id") == Some(frag));
            let by_name =
                || dom.nodes.iter().position(|n| n.tag == "a" && n.attr("name") == Some(frag));
            by_id.or_else(by_name).map(|node| dom.box_of(node, 1))
        })
    };
    match y {
        Some(y) => {
            super::scroll_by::apply_scroll(state, ScrollAct::To(y as i64));
            true
        }
        None => false,
    }
}
