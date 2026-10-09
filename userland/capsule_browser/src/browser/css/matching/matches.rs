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

use crate::browser::css::parse::parse_selectors;

use super::select::QUERY_STEPS;
use super::selector::matches_scoped;
use super::sibling::Siblings;

/// How far up a parent chain a walk goes before giving up. A tree a script
/// built can hold a cycle, and a walk up it would otherwise never end.
const MAX_ANCESTRY: u32 = 512;

/// Whether one node matches a selector list, as element.matches asks:
/// :scope is the node itself.
///
/// `select` answers this by walking the whole document and keeping the hits,
/// which is the wrong shape for a script asking about the node it already
/// has. Scripts ask constantly: event delegation is a `closest` call per
/// event, so the walk would run once per click over every node in the page.
pub fn matches(dom: &Dom, id: usize, selector: &str) -> bool {
    closest_within(dom, id, selector, 1).is_some()
}

/// The nearest node at or above `id` that matches, or none. :scope is `id`.
///
/// This is how a page turns a click on whatever was under the pointer into
/// the row, button or link the handler is about, so it runs on every event a
/// delegating listener sees. Its work shares one query budget.
pub fn closest(dom: &Dom, id: usize, selector: &str) -> Option<usize> {
    closest_within(dom, id, selector, MAX_ANCESTRY)
}

fn closest_within(dom: &Dom, id: usize, selector: &str, hops: u32) -> Option<usize> {
    let sels = parse_selectors(selector);
    let walk = Siblings::walk();
    let (mut at, mut spent) = (id, 0u64);
    for _ in 0..hops {
        let node = dom.nodes.get(at)?;
        if node.kind == NodeKind::Element {
            for s in &sels {
                let (hit, n) = matches_scoped(dom, &walk, id, at, s);
                spent += n as u64;
                if hit {
                    return Some(at);
                }
            }
        }
        if node.parent == at || spent > QUERY_STEPS {
            return None;
        }
        at = node.parent;
    }
    None
}
