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

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use crate::browser::css::selector::Selector;

/* The id a list asks for when it is exactly one bare `#id`, the query
 * pages make most often. */
pub(super) fn lone_id(sels: &[Selector]) -> Option<&str> {
    let [s] = sels else { return None };
    let k = &s.key;
    let bare = k.tag.is_none() && k.classes.is_empty() && k.attrs.is_empty() && k.pseudo.is_empty();
    (bare && s.element == 0 && s.ancestors.is_empty()).then_some(k.id.as_deref()?)
}

/* `#id` over the whole document by one scan of the node list, without
 * parsing the tree into order or building a table. None when two or more
 * elements carry the id, where tree order has to decide and the full walk
 * answers instead. */
pub(super) fn by_id(dom: &Dom, want: &str, limit: usize) -> Option<Vec<usize>> {
    let mut found: Option<usize> = None;
    for (i, n) in dom.nodes.iter().enumerate() {
        let hit = n.kind == NodeKind::Element
            && n.attrs.iter().any(|(k, v)| v == want && k.eq_ignore_ascii_case("id"));
        if hit && found.replace(i).is_some() {
            return None;
        }
    }
    Some(match found {
        Some(i) if limit > 0 && connected(dom, i) => vec![i],
        _ => Vec::new(),
    })
}

/* Whether the element hangs from the document: every hop up is listed by
 * the parent it names. A created or removed node is not. */
fn connected(dom: &Dom, mut id: usize) -> bool {
    for _ in 0..dom.nodes.len() {
        let Some(p) = dom.nodes.get(id).map(|n| n.parent) else { return false };
        if !dom.nodes.get(p).is_some_and(|pn| pn.children.contains(&id)) {
            return false;
        }
        if p == 0 {
            return true;
        }
        id = p;
    }
    false
}
