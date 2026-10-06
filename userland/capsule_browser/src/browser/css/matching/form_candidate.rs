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

use super::cx::Cx;
use super::form_disabled::disabled;
use super::form_kind::{input_kind, is_submit, typed};

/* Parent hops a datalist lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* A submittable control that is a candidate for constraint validation:
 * not disabled, not in a datalist, not a hidden, reset or button input,
 * not a readonly text field, and for a button only a submit button. */
pub(super) fn control(cx: &Cx, id: usize) -> bool {
    let Some(n) = cx.element(id) else { return false };
    let candidate = match n.tag.as_str() {
        "input" => {
            let kind = input_kind(n);
            !(matches!(kind, "hidden" | "reset" | "button")
                || (typed(kind) && n.attr("readonly").is_some()))
        }
        "textarea" => n.attr("readonly").is_none(),
        "select" => true,
        "button" => is_submit(n),
        _ => false,
    };
    candidate && !disabled(cx.dom, id) && !in_datalist(cx, id)
}

fn in_datalist(cx: &Cx, id: usize) -> bool {
    let mut node = id;
    for _ in 0..MAX_HOPS {
        match cx.parent_el(node) {
            Some(p) if cx.dom.nodes[p].tag == "datalist" => return true,
            Some(p) => node = p,
            None => return false,
        }
    }
    false
}
