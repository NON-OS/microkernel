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

use alloc::format;
use alloc::string::String;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* Nesting kept when serializing; deeper content is left out. */
const MAX_DEPTH: u32 = 256;

/* Serialize a DOM subtree back to SVG markup so an inline <svg> can be handed
 * to the same rasterizer that decodes an <img src=*.svg>. The svg namespace
 * is asserted on the root so a fragment lifted from HTML stands alone, and
 * `size`, the viewport CSS layout gave the box, replaces the root's own
 * width and height: CSS sizes override those attributes, and the rasterizer
 * maps a missing viewBox one unit to one px of this size. */
pub(super) fn serialize_svg(dom: &Dom, id: usize, size: Size) -> String {
    let mut out = String::new();
    write_node(dom, id, &mut out, 0, Some(size));
    out
}

/* A viewport size in px, when layout knows it. */
type Size = Option<(i32, i32)>;

/* `root` is Some for the subtree's root, holding the size it is given. */
fn write_node(dom: &Dom, id: usize, out: &mut String, depth: u32, root: Option<Size>) {
    if depth > MAX_DEPTH {
        return;
    }
    let Some(node) = dom.nodes.get(id) else { return };
    match node.kind {
        NodeKind::Text => out.push_str(node.text.trim()),
        NodeKind::Element => {
            out.push('<');
            out.push_str(&node.tag);
            if root.is_some() && node.attr("xmlns").is_none() {
                out.push_str(" xmlns=\"http://www.w3.org/2000/svg\"");
            }
            let sized = root.flatten();
            if let Some((w, h)) = sized {
                out.push_str(&format!(" width=\"{}\" height=\"{}\"", w.max(0), h.max(0)));
            }
            let own = |k: &str| sized.is_none() || (k != "width" && k != "height");
            for (k, v) in node.attrs.iter().filter(|(k, _)| own(k)) {
                for part in [" ", k, "=\"", v, "\""] {
                    out.push_str(part);
                }
            }
            out.push('>');
            for &ch in &node.children {
                write_node(dom, ch, out, depth + 1, None);
            }
            for part in ["</", &node.tag, ">"] {
                out.push_str(part);
            }
        }
        NodeKind::Document => {}
    }
}
