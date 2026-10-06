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

//! The control a `<label>` names, so a click on the label is a click on it.
//!
//! A click on the words beside a checkbox did nothing, and on most forms
//! those words are the larger target, and the one a reader aims at.

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

/* Parent hops the walk up to a label climbs, and nodes the walk down into
 * one visits; a script can build a deep tree, or a loop. */
const MAX_HOPS: u32 = 64;
const MAX_WALK: u32 = 4096;

/// The control a click on `node` reaches through a label: the nearest
/// `<label>` at or above `node`, then the element its `for` names, else
/// the first control inside it (HTML 4.10.4). None when no label is
/// there, or it names nothing a label can stand for.
pub fn label_control(dom: &Dom, node: usize) -> Option<usize> {
    let label = enclosing_label(dom, node)?;
    match dom.nodes[label].attr("for") {
        Some(id) => {
            let target = dom
                .nodes
                .iter()
                .position(|n| n.kind == NodeKind::Element && n.attr("id") == Some(id))?;
            labelable(&dom.nodes[target]).then_some(target)
        }
        None => first_control(dom, label),
    }
}

/* The label at or above `node`, if there is one. */
fn enclosing_label(dom: &Dom, node: usize) -> Option<usize> {
    let mut cur = node;
    for _ in 0..MAX_HOPS {
        let n = dom.nodes.get(cur).filter(|_| cur != 0)?;
        if n.kind == NodeKind::Element && n.tag == "label" {
            return Some(cur);
        }
        cur = n.parent;
    }
    None
}

/* The first labelable element inside `label`, in document order. */
fn first_control(dom: &Dom, label: usize) -> Option<usize> {
    let mut stack: alloc::vec::Vec<usize> =
        dom.nodes[label].children.iter().rev().copied().collect();
    let mut walked = 0u32;
    while let Some(id) = stack.pop() {
        walked += 1;
        let n = dom.nodes.get(id)?;
        if walked > MAX_WALK {
            return None;
        }
        if labelable(n) {
            return Some(id);
        }
        stack.extend(n.children.iter().rev().copied());
    }
    None
}

/* What a label can stand for, of what this browser draws: a button, a
 * select, a textarea, or an input that is not hidden. */
fn labelable(n: &Node) -> bool {
    if n.kind != NodeKind::Element {
        return false;
    }
    match n.tag.as_str() {
        "button" | "select" | "textarea" => true,
        "input" => !n.attr("type").is_some_and(|t| t.eq_ignore_ascii_case("hidden")),
        _ => false,
    }
}
