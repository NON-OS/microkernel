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

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

/* The nodes of `node`'s declarative shadow root, when it is a shadow host:
 * what its <template shadowrootmode> child holds. The DOM keeps a
 * template's contents as the template's children today; this is the one
 * place that reads them, so when templates carry a content fragment only
 * this has to follow it. */
fn shadow_root<'a>(dom: &'a Dom, node: &Node) -> Option<&'a [usize]> {
    node.children.iter().find_map(|&t| {
        let c = dom.nodes.get(t)?;
        let shadow = c.kind == NodeKind::Element && c.tag == "template";
        (shadow && c.attr("shadowrootmode").is_some()).then_some(c.children.as_slice())
    })
}

/* Which children of `node` make no box: all but the <summary> of a closed
 * <details>, until it opens, and a declarative shadow host's light
 * children aimed at named slots. The host renders its shadow tree, which
 * layout skips, so slotted ones (dropdown panels) would paint over the
 * page with no component to place them; default-slot children (button
 * labels) still render. */
pub(in super::super) fn hidden_child(dom: &Dom, node: &Node) -> impl Fn(&Node) -> bool {
    let closed_details = node.tag == "details" && node.attr("open").is_none();
    let shadow_host = shadow_root(dom, node).is_some();
    move |c: &Node| {
        let summary = c.kind == NodeKind::Element && c.tag == "summary";
        let slotted = c.attr("slot").is_some_and(|s| !s.is_empty());
        (closed_details && !summary) || (shadow_host && slotted)
    }
}
