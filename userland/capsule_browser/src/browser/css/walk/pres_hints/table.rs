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

use crate::browser::dom::node::Node;
use crate::browser::dom::Dom;

use super::attrs::{border, int, table_of};
use super::legacy::{align, bgcolor, hint, length, Hints};

/* The hints of a table, a row or row group, or a cell. A table's border
 * attribute draws its own outset border and a one-pixel inset border
 * on each of its cells; cellpadding pads its cells and cellspacing sets
 * the spacing between them. */
pub(super) fn hints(dom: &Dom, node: &Node, out: &mut Hints) {
    bgcolor(node, "bgcolor", "background-color", out);
    length(node, "height", "height", out);
    let valign = node.attr("valign").map(|v| v.trim().to_ascii_lowercase());
    if let Some(v) =
        valign.filter(|v| matches!(v.as_str(), "top" | "middle" | "bottom" | "baseline"))
    {
        hint(out, "vertical-align", v);
    }
    match node.tag.as_str() {
        "table" => {
            length(node, "width", "width", out);
            if let Some(b) = border(node).filter(|&b| b > 0) {
                hint(out, "border", format!("{b}px outset #808080"));
            }
            if let Some(n) = int(node, "cellspacing") {
                hint(out, "border-spacing", format!("{n}px"));
            }
            match node.attr("align").map(|a| a.trim().to_ascii_lowercase()).as_deref() {
                Some("center") => {
                    hint(out, "margin-left", "auto");
                    hint(out, "margin-right", "auto");
                }
                Some(side @ ("left" | "right")) => hint(out, "float", side),
                _ => {}
            }
        }
        "td" | "th" => {
            length(node, "width", "width", out);
            align(node, out);
            if node.attr("nowrap").is_some() {
                hint(out, "white-space", "nowrap");
            }
            let table = table_of(dom, node);
            if let Some(n) = table.and_then(|t| int(t, "cellpadding")) {
                hint(out, "padding", format!("{n}px"));
            }
            if table.and_then(border).is_some_and(|b| b > 0) {
                hint(out, "border", "1px inset #808080");
            }
        }
        _ => align(node, out),
    }
}
