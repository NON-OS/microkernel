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

/* :empty: no element children and no text, not even whitespace. */
pub(super) fn empty(dom: &Dom, id: usize) -> bool {
    dom.nodes.get(id).is_some_and(|n| {
        n.children.iter().all(|&ch| {
            dom.nodes.get(ch).is_none_or(|c| c.kind == NodeKind::Text && c.text.is_empty())
        })
    })
}

/* The document element: an element the document root lists as its child. A
 * detached element also points at the root but is not listed there. */
pub(super) fn is_root(dom: &Dom, id: usize) -> bool {
    let (Some(n), Some(doc)) = (dom.nodes.get(id), dom.nodes.first()) else {
        return false;
    };
    id != 0 && n.kind == NodeKind::Element && n.parent == 0 && doc.children.contains(&id)
}

/* :link and :any-link: an a or area element with an href. :visited never
 * holds, so every link is unvisited. */
pub(super) fn is_link(dom: &Dom, id: usize) -> bool {
    dom.nodes
        .get(id)
        .is_some_and(|n| matches!(n.tag.as_str(), "a" | "area") && n.attr("href").is_some())
}

/* :defined: not an autonomous custom element, since no script here can
 * define one. A custom element name holds a hyphen and starts with a
 * lower-case ASCII letter; the eight hyphenated names SVG and MathML use are
 * not custom. */
pub(super) fn defined(dom: &Dom, id: usize) -> bool {
    const RESERVED: [&str; 8] = [
        "annotation-xml",
        "color-profile",
        "font-face",
        "font-face-src",
        "font-face-uri",
        "font-face-format",
        "font-face-name",
        "missing-glyph",
    ];
    dom.nodes.get(id).is_some_and(|n| {
        let t = n.tag.as_str();
        let custom = t.contains('-') && t.starts_with(|c: char| c.is_ascii_lowercase());
        !custom || RESERVED.contains(&t)
    })
}

/* :open: a details or dialog element carrying the open attribute. */
pub(super) fn is_open(dom: &Dom, id: usize) -> bool {
    dom.nodes
        .get(id)
        .is_some_and(|n| matches!(n.tag.as_str(), "details" | "dialog") && n.attr("open").is_some())
}
