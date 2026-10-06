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

use crate::browser::css::computed::Computed;
use crate::browser::dom::node::Node;

/* The largest span the HTML table model keeps. */
const MAX_SPAN: u16 = 1000;

/* A table cell's colspan and rowspan attributes, 1 when absent or not a
 * number (a rowspan of 0 spans to the end of its group in HTML; this
 * grid has no groups, so it spans one row). */
pub(super) fn spans(c: &mut Computed, node: &Node) {
    if c.is_table_cell {
        let span = |a: &str| node.attr(a).and_then(|v| v.trim().parse::<u16>().ok());
        c.table.col_span = span("colspan").unwrap_or(1).clamp(1, MAX_SPAN);
        c.table.row_span = span("rowspan").unwrap_or(1).clamp(1, MAX_SPAN);
    }
}
