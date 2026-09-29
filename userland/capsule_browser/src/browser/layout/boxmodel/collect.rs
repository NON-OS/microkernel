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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;
use crate::browser::dom::node::NodeKind;

use super::contexts::collect_text::text_child;
use super::contexts::shadow::hidden_child;
use super::element::element;
use super::tree::BoxNode;
use super::walk::{ElementIn, Walk};

pub(super) const MAX_DEPTH: u32 = 400;
const MAX_BOXES: usize = 20_000;

/* Build the box children of node `id`, recursing into elements. Text goes
 * through the white-space policy; elements through the per-tag
 * dispatcher. The li ordinal counts list items in this container for
 * ordered-list markers. */
pub(super) fn collect(
    w: &mut Walk,
    id: usize,
    parent: &Computed,
    link: &Option<String>,
    depth: u32,
) -> Vec<BoxNode> {
    let mut out: Vec<BoxNode> = Vec::new();
    let (dom, styles) = (w.dom, w.styles);
    let Some(node) = dom.nodes.get(id).filter(|_| depth <= MAX_DEPTH) else { return out };
    let (mut ordinal, hidden) = (0u32, hidden_child(dom, node));
    for &ch in &node.children {
        if *w.count >= MAX_BOXES {
            break;
        }
        let Some(c) = dom.nodes.get(ch).filter(|c| !hidden(c)) else { continue };
        match c.kind {
            NodeKind::Text => text_child(w, &c.text, ch, parent, link, &mut out),
            /* display: contents makes no box: the element's children join
             * this run directly (grid shells rely on this). The style is
             * borrowed, not copied, to keep this recursive frame small. */
            NodeKind::Element if styles[ch].is_contents && !styles[ch].display_none => {
                let spliced = collect(w, ch, &styles[ch], link, depth + 1);
                out.extend(spliced);
            }
            NodeKind::Element => {
                ordinal += (c.tag == "li") as u32;
                let item = ElementIn { c, ch, parent_tag: &node.tag, ordinal };
                element(w, &item, parent, link, depth, &mut out);
            }
            /* The document node, and any kind that draws nothing. */
            _ => {}
        }
    }
    out
}
