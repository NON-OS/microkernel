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

/* The table properties and attributes a box carries; only tables read. */
#[derive(Clone, Copy)]
pub struct TableStyle {
    /* vertical-align, one of the VA_ values; a table cell takes its row's
     * and row group's unless it declares its own. */
    pub valign: u8,
    /* border-spacing between a table's cells in px, and whether
     * border-collapse: collapse joins the cell borders instead;
     * inherited. */
    pub border_spacing: u16,
    pub border_collapse: bool,
    /* A table cell's colspan and rowspan attributes, at least 1. */
    pub col_span: u16,
    pub row_span: u16,
    /* text-align: -webkit-center (<center>, align=center): block-level
     * tables inside centre themselves as well as the text; inherited. */
    pub center_blocks: bool,
    /* display: table-caption, the only child of a table laid out as its
     * caption. */
    pub caption: bool,
}

impl TableStyle {
    pub const VA_BASELINE: u8 = 0;
    pub const VA_TOP: u8 = 1;
    pub const VA_MIDDLE: u8 = 2;
    pub const VA_BOTTOM: u8 = 3;

    pub const INITIAL: TableStyle = TableStyle {
        valign: TableStyle::VA_BASELINE,
        border_spacing: 0,
        border_collapse: false,
        col_span: 1,
        row_span: 1,
        center_blocks: false,
        caption: false,
    };

    /* How much of a cell's spare height sits above its content, in
     * halves: none at the top (and baseline), one in the middle, both at
     * the bottom. */
    pub fn slack_halves(&self) -> i32 {
        match self.valign {
            TableStyle::VA_MIDDLE => 1,
            TableStyle::VA_BOTTOM => 2,
            _ => 0,
        }
    }

    /* A child's start: the inherited properties carried over. */
    pub fn inherit(&self) -> TableStyle {
        TableStyle {
            border_spacing: self.border_spacing,
            border_collapse: self.border_collapse,
            center_blocks: self.center_blocks,
            ..TableStyle::INITIAL
        }
    }
}
