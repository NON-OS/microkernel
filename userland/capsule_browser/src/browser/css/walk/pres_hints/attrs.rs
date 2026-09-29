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

use crate::browser::dom::node::Node;
use crate::browser::dom::Dom;

/* The table a cell sits in, through its row and any row group. */
pub(super) fn table_of<'d>(dom: &'d Dom, cell: &Node) -> Option<&'d Node> {
    let mut at = cell.parent;
    for _ in 0..3 {
        let n = dom.nodes.get(at)?;
        if n.tag == "table" {
            return Some(n);
        }
        at = n.parent;
    }
    None
}

/* A table's border attribute: its pixel width, or 1 when it is empty. */
pub(super) fn border(table: &Node) -> Option<u32> {
    let v = table.attr("border")?.trim();
    Some(if v.is_empty() { 1 } else { v.parse::<u32>().ok()?.min(64) })
}

pub(super) fn int(node: &Node, attr: &str) -> Option<u32> {
    node.attr(attr)?.trim().trim_end_matches("px").parse::<u32>().ok().map(|n| n.min(512))
}

/* <body marginwidth marginheight>: its margins, as old pages set them. */
pub(super) const BODY_MARGINS: [(&str, &str); 4] = [
    ("marginwidth", "margin-left"),
    ("marginwidth", "margin-right"),
    ("marginheight", "margin-top"),
    ("marginheight", "margin-bottom"),
];
