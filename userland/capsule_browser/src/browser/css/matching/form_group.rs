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

use super::cx::Cx;
use super::form_kind::{input_kind, is_submit};
use super::tree_scan::find_under;

/* Parent hops a form lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* The nearest form element above `id`. A form= attribute pointing a
 * control at a form elsewhere in the document is not followed. */
pub(super) fn form_owner(dom: &Dom, id: usize) -> Option<usize> {
    let mut node = dom.nodes.get(id)?.parent;
    for _ in 0..MAX_HOPS {
        let n = dom.nodes.get(node).filter(|_| node != 0)?;
        if n.tag == "form" {
            return Some(node);
        }
        node = n.parent;
    }
    None
}

pub(super) fn first_submit(cx: &Cx, form: usize) -> Option<usize> {
    find_under(cx, form, |d| cx.element(d).is_some_and(is_submit))
}

/* Whether the radio's group has a checked member (HTML 4.10.5.1.18): the
 * radios with the same non-empty name and the same form owner, or none. A
 * radio without a name is a group of one. */
pub(super) fn group_checked(cx: &Cx, id: usize) -> bool {
    let Some(n) = cx.element(id) else { return false };
    let Some(name) = n.attr("name").filter(|s| !s.is_empty()) else {
        return n.attr("checked").is_some();
    };
    let owner = form_owner(cx.dom, id);
    let member = |d: usize| {
        cx.element(d).is_some_and(|e| {
            e.tag == "input"
                && input_kind(e) == "radio"
                && e.attr("name") == Some(name)
                && e.attr("checked").is_some()
        }) && form_owner(cx.dom, d) == owner
    };
    find_under(cx, owner.unwrap_or(0), member).is_some()
}
