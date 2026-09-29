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

use crate::browser::css::selector::{is_space, Simple};
use crate::browser::dom::node::Node;

use super::cost::{lookup, ITEM};
use super::cx::Cx;

/* Every class the compound names is on the element. With a table the keys
 * are compared first and the class value is read only to confirm keys that
 * all matched; without one the value is split on ASCII whitespace byte by
 * byte, as HTML splits it, with no UTF-8 decoding. Tokens walked and bytes
 * read are paid for before the walk. */
pub(super) fn classes_match(cx: &Cx, id: usize, node: &Node, s: &Simple) -> bool {
    if s.classes.is_empty() {
        return true;
    }
    let toks = cx.sib.tab().map(|t| t.classes(id));
    let names = s.classes.len();
    let walk = toks.map_or(0, |t| t.len().saturating_mul(ITEM).saturating_mul(names));
    if let Some(toks) = toks {
        if !cx.charge(walk) || !s.class_keys.iter().all(|k| toks.iter().any(|t| t.key == *k)) {
            return false;
        }
    }
    let Some(value) = node.attr("class").map(str::as_bytes) else {
        return false;
    };
    let read = match toks {
        Some(_) => walk.saturating_add(s.classes.iter().map(|c| c.len()).sum()),
        None => value.len().saturating_mul(names),
    };
    if !cx.charge(lookup(node).saturating_add(read)) {
        return false;
    }
    s.classes.iter().zip(&s.class_keys).all(|(name, key)| match toks {
        Some(toks) => toks.iter().any(|t| {
            t.key == *key && value.get(t.start as usize..t.end as usize) == Some(name.as_bytes())
        }),
        None => value.split(|b| is_space(*b)).any(|w| w == name.as_bytes()),
    })
}

/* The element's id is the compound's; the table's key rejects first. */
pub(super) fn id_matches(cx: &Cx, id: usize, node: &Node, s: &Simple) -> bool {
    if cx.sib.tab().is_some_and(|t| t.id_key(id) != s.id_key) {
        return false;
    }
    let want = s.id.as_deref().map_or(0, str::len);
    cx.charge(lookup(node).saturating_add(want)) && node.attr("id") == s.id.as_deref()
}
