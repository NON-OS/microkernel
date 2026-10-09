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

//! A click on a checkbox or a radio button.
//!
//! Both used to take the keyboard as a text field does: a click never
//! checked one, and keys typed after it rewrote the value the box would
//! send. A checkbox now flips, and a radio button is checked and the others
//! of its group are not, as the `checked` attribute that :checked, a submit
//! and a script all read.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* Parent hops a form lookup climbs; a script can build a loop. */
const MAX_HOPS: u32 = 512;

/// What a toggle changed: each control it touched and whether it was
/// checked before, so a click a listener cancels can be undone.
pub type Prior = Vec<(usize, bool)>;

/// Check or uncheck the checkbox or radio button `id`, as a click does
/// before the page's listeners see it. None when it is neither, or is
/// disabled, and nothing changed.
pub fn toggle(dom: &mut Dom, id: usize) -> Option<Prior> {
    let n = dom.nodes.get(id)?;
    let kind = n.attr("type").unwrap_or("").to_ascii_lowercase();
    if n.tag != "input" || n.attr("disabled").is_some() {
        return None;
    }
    let was = n.attr("checked").is_some();
    let mut prior: Prior = alloc::vec![(id, was)];
    match kind.as_str() {
        "checkbox" if was => dom.remove_attr(id, "checked"),
        "checkbox" => dom.set_attr(id, "checked", String::new()),
        "radio" => {
            for other in group(dom, id) {
                prior.push((other, dom.nodes[other].attr("checked").is_some()));
                dom.remove_attr(other, "checked");
            }
            dom.set_attr(id, "checked", String::new());
        }
        _ => return None,
    }
    Some(prior)
}

/// Put back what `toggle` changed, for a click a listener cancelled.
pub fn restore(dom: &mut Dom, prior: &[(usize, bool)]) {
    for &(id, checked) in prior {
        match checked {
            true => dom.set_attr(id, "checked", String::new()),
            false => dom.remove_attr(id, "checked"),
        }
    }
}

/* The other radio buttons of `id`'s group: the same non-empty name and the
 * same form owner (HTML 4.10.5.1.18). One without a name is alone. */
fn group(dom: &Dom, id: usize) -> Vec<usize> {
    let Some(name) = dom.nodes[id].attr("name").filter(|s| !s.is_empty()) else {
        return Vec::new();
    };
    let owner = form_owner(dom, id);
    (1..dom.nodes.len())
        .filter(|&d| d != id)
        .filter(|&d| {
            let e = &dom.nodes[d];
            e.kind == NodeKind::Element
                && e.tag == "input"
                && e.attr("type").is_some_and(|t| t.eq_ignore_ascii_case("radio"))
                && e.attr("name") == Some(name)
                && form_owner(dom, d) == owner
        })
        .collect()
}

/* The nearest form above `id`, or None for a control outside any form. */
fn form_owner(dom: &Dom, id: usize) -> Option<usize> {
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
