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

//! The budgets that bound what one page can make the parser hold: attributes
//! per tag, attributes per document, and nodes per document.

use crate::browser::dom::limits::{MAX_ATTRS, MAX_NODES, TRUNC_ATTRS, TRUNC_NODES};
use crate::browser::html::tokenizer::MAX_TAG_ATTRS;
use crate::shape::{all, check, doc};

#[test]
fn a_tag_keeps_its_first_attributes_up_to_the_cap() {
    let attrs: String = (0..100).map(|i| format!(" a{i}={i}")).collect();
    let d = doc(&format!("<p{attrs} a0=dup>x"));
    let p = &d.nodes[all(&d, "p")[0]];
    assert_eq!(p.attrs.len(), MAX_TAG_ATTRS);
    assert_eq!(p.attr("a0"), Some("0"));
    assert_ne!(d.truncated & TRUNC_ATTRS, 0);
}

#[test]
fn the_document_attribute_budget_holds() {
    let tag = format!("<i{}></i>", (0..64).map(|i| format!(" a{i}")).collect::<String>());
    let d = doc(&tag.repeat(4_000));
    let held: usize = d.nodes.iter().map(|n| n.attrs.len()).sum();
    assert_eq!(held, MAX_ATTRS);
    assert_ne!(d.truncated & TRUNC_ATTRS, 0);
}

#[test]
fn the_node_cap_stops_growth_and_says_so() {
    let d = doc(&"<i>x</i>".repeat(40_000));
    check(&d);
    assert!(d.nodes.len() <= MAX_NODES);
    assert_ne!(d.truncated & TRUNC_NODES, 0);
    assert_eq!(doc("<i>x</i>").truncated, 0);
}
