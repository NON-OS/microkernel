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
use crate::browser::dom::quirks::Quirks;

/* What a table part starts from before its declarations apply. A row and a
 * cell align vertically as their row group does unless they declare
 * otherwise (vertical-align: inherit in the UA sheet). In quirks mode a
 * table does not inherit text alignment, white space, line height or font
 * size, weight and style (HTML, rendering, the quirks-mode table rule), so
 * a table inside <center> keeps its cells' text at the start. */
pub(super) fn table_start(c: &mut Computed, parent: &Computed, tag: &str, quirks: Quirks) {
    if matches!(tag, "tr" | "td" | "th") {
        c.table.valign = parent.table.valign;
    }
    /* -webkit-center is a text alignment too: the table keeps the centring
     * it was given, the tables nested in its cells no longer take it. */
    if quirks == Quirks::Full && matches!(tag, "thead" | "tbody" | "tfoot" | "tr") {
        c.table.center_blocks = false;
    }
    if tag == "table" && quirks == Quirks::Full {
        let r = Computed::root();
        (c.text_align, c.white_space) = (r.text_align, r.white_space);
        (c.line_height_px, c.line_ratio) = (r.line_height_px, r.line_ratio);
        (c.font_size_px, c.font_px) = (r.font_size_px, r.font_px);
        (c.bold, c.italic) = (r.bold, r.italic);
        c.font_key = crate::browser::fonts::weighted(c.font_key, 400);
    }
}
