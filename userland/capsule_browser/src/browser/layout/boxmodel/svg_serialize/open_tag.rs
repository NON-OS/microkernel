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

use crate::browser::dom::node::Node;

use super::Size;

/* `<tag attrs>` of one element. The svg namespace is asserted on the root,
 * whose width and height become `root`'s size when layout settled one.
 * `style`, the cascade's resolved paint, goes before the element's own
 * inline style: the rasterizer takes a property's first value, and the
 * cascade already ranked that inline style against the page's rules. */
pub(super) fn open_tag(node: &Node, style: Option<&str>, root: Option<Size>, out: &mut String) {
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
    let skip = |k: &str| style.is_some() && k.eq_ignore_ascii_case("style");
    for (k, v) in node.attrs.iter().filter(|(k, _)| own(k) && !skip(k)) {
        for part in [" ", k, "=\"", v, "\""] {
            out.push_str(part);
        }
    }
    if let Some(s) = style {
        let inline = node.attr("style").unwrap_or("");
        for part in [" style=\"", s, ";", inline, "\""] {
            out.push_str(part);
        }
    }
    out.push('>');
}
