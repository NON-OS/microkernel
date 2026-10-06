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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* Parent hops a fieldset lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/* HTML's "actually disabled": the disabled attribute, or (for a form
 * control or fieldset) a place inside a disabled fieldset other than in its
 * first legend; an option is also disabled by a disabled optgroup parent. */
pub(super) fn disabled(dom: &Dom, id: usize) -> bool {
    let Some(n) = dom.nodes.get(id) else { return false };
    let own = n.attr("disabled").is_some();
    match n.tag.as_str() {
        "button" | "input" | "select" | "textarea" | "fieldset" => {
            own || in_disabled_fieldset(dom, id)
        }
        "optgroup" => own,
        "option" => {
            let group = dom.nodes.get(n.parent).filter(|p| p.tag == "optgroup");
            own || group.is_some_and(|g| g.attr("disabled").is_some())
        }
        _ => false,
    }
}

fn in_disabled_fieldset(dom: &Dom, id: usize) -> bool {
    let (mut child, mut node) = (id, dom.nodes[id].parent);
    for _ in 0..MAX_HOPS {
        let Some(n) = dom.nodes.get(node).filter(|_| node != 0 && node != child) else {
            return false;
        };
        if n.tag == "fieldset" && n.attr("disabled").is_some() {
            let legend = n.children.iter().copied().find(|&c| {
                dom.nodes.get(c).is_some_and(|l| l.kind == NodeKind::Element && l.tag == "legend")
            });
            if legend != Some(child) {
                return true;
            }
        }
        (child, node) = (node, n.parent);
    }
    false
}
