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

//! What the proofs read back from a parsed tree: the markup under an
//! element, the elements with a name, the nesting depth, and the arena's
//! structural invariants.

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::{self, Dom};

/// Parse a whole document from a string.
pub fn doc(html: &str) -> Dom {
    dom::parse(html.as_bytes())
}

/// Every HTML or foreign element named `tag`, in document order.
pub fn all(d: &Dom, tag: &str) -> Vec<usize> {
    let hit = |&i: &usize| d.nodes[i].kind == NodeKind::Element && d.nodes[i].tag == tag;
    (0..d.nodes.len()).filter(hit).collect()
}

/// The markup inside the first element named `tag`.
pub fn inner(d: &Dom, tag: &str) -> String {
    let id = *all(d, tag).first().unwrap_or_else(|| panic!("no <{tag}> in the tree"));
    d.inner_html(id)
}

/// The markup inside the body.
pub fn body(html: &str) -> String {
    let d = doc(html);
    check(&d);
    inner(&d, "body")
}

/// The deepest element, counting the document as depth 0.
pub fn depth(d: &Dom) -> usize {
    let mut best = 0;
    let mut stack = vec![(0usize, 0usize)];
    while let Some((id, at)) = stack.pop() {
        best = best.max(at);
        stack.extend(d.nodes[id].children.iter().map(|&c| (c, at + 1)));
    }
    best
}

/// The arena invariants the engine relies on: every node but the document
/// sits once under a parent that precedes it, and no text node is empty.
pub fn check(d: &Dom) {
    let mut seen = vec![0u8; d.nodes.len()];
    seen[0] = 1;
    for (id, n) in d.nodes.iter().enumerate() {
        for &c in &n.children {
            assert!(c > id, "child {c} precedes its parent {id}");
            assert_eq!(d.nodes[c].parent, id, "child {c} names another parent");
            seen[c] += 1;
        }
        let empty_text = n.kind == NodeKind::Text && n.text.is_empty();
        assert!(!empty_text, "empty text node {id}");
    }
    let bad = seen.iter().position(|&s| s != 1);
    assert!(bad.is_none(), "node {bad:?} is outside the tree or listed twice");
}
