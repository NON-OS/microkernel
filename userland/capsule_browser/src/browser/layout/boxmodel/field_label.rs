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

use alloc::string::{String, ToString};

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

/* Visible text for a form widget: the value (or placeholder) of an input,
 * the selected option of a select (else its first). Empty means blank. */
pub(super) fn field_label(dom: &Dom, id: usize) -> String {
    let Some(node) = dom.nodes.get(id) else {
        return String::new();
    };
    match node.tag.as_str() {
        "input" => {
            node.attr("value").or_else(|| node.attr("placeholder")).unwrap_or("").to_string()
        }
        "select" => {
            let chosen = options(dom, node).find(|o| o.attr("selected").is_some());
            chosen.or_else(|| options(dom, node).next()).map(|o| text(dom, o)).unwrap_or_default()
        }
        _ => String::new(),
    }
}

/* The longest option label of a select, which its width must fit. */
pub(super) fn widest_option(dom: &Dom, id: usize) -> String {
    let Some(node) = dom.nodes.get(id) else {
        return String::new();
    };
    options(dom, node).map(|o| text(dom, o)).max_by_key(|t| t.chars().count()).unwrap_or_default()
}

/* The <option> elements of a select, directly or inside an <optgroup>. */
fn options<'a>(dom: &'a Dom, select: &'a Node) -> impl Iterator<Item = &'a Node> + 'a {
    let kids = move |n: &'a Node| n.children.iter().filter_map(move |&c| dom.nodes.get(c));
    kids(select)
        .flat_map(move |c| {
            let group = (c.tag == "optgroup").then(|| kids(c)).into_iter().flatten();
            core::iter::once(c).chain(group)
        })
        .filter(|o| o.kind == NodeKind::Element && o.tag == "option")
}

/* An option's text, trimmed. */
fn text(dom: &Dom, option: &Node) -> String {
    let mut out = String::new();
    for &t in &option.children {
        if let Some(tn) = dom.nodes.get(t).filter(|n| n.kind == NodeKind::Text) {
            out.push_str(&tn.text);
        }
    }
    out.trim().to_string()
}
