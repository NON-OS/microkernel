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

use alloc::vec::Vec;

use crate::browser::dom::Dom;

use crate::browser::css::parse::parse_selectors;

use super::select_id::{by_id, lone_id};
use super::selector::matches_scoped;
use super::sibling::Siblings;
use super::tree_walk::Under;

/* Compound tests one query may spend. A selector costing more than this
 * over the tree stops the walk there and the query returns what it found
 * so far, so a hostile querySelectorAll ends in milliseconds instead of
 * holding the page. */
pub(super) const QUERY_STEPS: u64 = 4_000_000;

/* A query under a subtree larger than this builds the sibling table for a
 * positional selector; a smaller one walks. */
const TABLE_AT: usize = 1024;

/* Element ids matching a selector list, in tree order, capped at `limit`:
 * document.querySelectorAll. */
pub fn select(dom: &Dom, selector: &str, limit: usize) -> Vec<usize> {
    select_in(dom, 0, selector, limit)
}

/* The elements under `root` (0 for the whole document) matching a selector
 * list, in tree order, capped at `limit`. `root` itself is not a candidate
 * and :scope is `root` (the document element when 0), as for
 * element.querySelectorAll. Only root's subtree is walked, and detached
 * nodes are never found. */
pub fn select_in(dom: &Dom, root: usize, selector: &str, limit: usize) -> Vec<usize> {
    let sels = parse_selectors(selector);
    if sels.is_empty() || limit == 0 || root >= dom.nodes.len() {
        return Vec::new();
    }
    if let Some(hits) = lone_id(&sels).filter(|_| root == 0).and_then(|w| by_id(dom, w, limit)) {
        return hits;
    }
    let positional = sels.iter().any(|s| s.positional);
    let big = root == 0 || Under::new(dom, root).nth(TABLE_AT).is_some();
    let sib = if positional && big { Siblings::table(dom) } else { Siblings::walk() };
    let (mut out, mut spent) = (Vec::new(), 0u64);
    for id in Under::new(dom, root) {
        for s in &sels {
            let (hit, n) = matches_scoped(dom, &sib, root, id, s);
            spent += n as u64;
            if hit {
                out.push(id);
                break;
            }
        }
        if out.len() >= limit || spent > QUERY_STEPS {
            break;
        }
    }
    out
}
