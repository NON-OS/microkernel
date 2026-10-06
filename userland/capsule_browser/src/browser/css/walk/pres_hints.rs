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

mod attrs;
mod font;
mod legacy;
mod table;

use alloc::vec::Vec;

use crate::browser::css::decl::Decl;
use crate::browser::dom::Dom;

use legacy::{align, bgcolor, hint, length, Hints};

/* Presentational hints of element `id`, after the rendering section of
 * the HTML standard: the legacy attributes that style an element (body
 * text and link colours, bgcolor, width and height, align and valign,
 * table border, cellpadding and cellspacing, <font>), as author-level
 * declarations of specificity zero that every author rule overrides. */
pub(in crate::browser::css) fn pres_hints(dom: &Dom, id: usize, link: Option<&str>) -> Vec<Decl> {
    let mut out: Hints = Vec::new();
    let Some(node) = dom.nodes.get(id) else { return out };
    match node.tag.as_str() {
        "body" => {
            bgcolor(node, "bgcolor", "background-color", &mut out);
            bgcolor(node, "text", "color", &mut out);
            for (attr, prop) in attrs::BODY_MARGINS {
                length(node, attr, prop, &mut out);
            }
        }
        "table" | "td" | "th" | "tr" | "tbody" | "thead" | "tfoot" => {
            table::hints(dom, node, &mut out)
        }
        "font" => {
            bgcolor(node, "color", "color", &mut out);
            if let Some(face) = node.attr("face") {
                hint(&mut out, "font-family", face);
            }
            if let Some(px) = node.attr("size").and_then(font::font_size) {
                hint(&mut out, "font-size", px);
            }
        }
        "div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "caption" | "legend" => {
            align(node, &mut out)
        }
        "hr" => {
            length(node, "width", "width", &mut out);
            length(node, "size", "height", &mut out);
            bgcolor(node, "color", "border-color", &mut out);
        }
        /* <body link> colours every link; the walk reads it once. */
        "a" if node.attr("href").is_some() => {
            if let Some(c) = link.and_then(legacy::color) {
                hint(&mut out, "color", c);
            }
        }
        _ => {}
    }
    out
}
