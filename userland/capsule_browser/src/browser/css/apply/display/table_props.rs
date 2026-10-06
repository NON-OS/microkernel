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

use crate::browser::css::computed::{Computed, TableStyle};
use crate::browser::css::parse_px::parse_px;

/* The largest border-spacing kept; more is a layout trick, not a gap. */
const MAX_SPACING: u32 = 512;

/* The table properties: vertical-align (read by table cells),
 * border-spacing (its first, horizontal, length) and border-collapse. */
pub(super) fn apply_table(c: &mut Computed, name: &str, value: &str) -> bool {
    let v = value.trim();
    match name {
        "vertical-align" => {
            c.table.valign = match v {
                "top" | "text-top" => TableStyle::VA_TOP,
                "middle" => TableStyle::VA_MIDDLE,
                "bottom" | "text-bottom" => TableStyle::VA_BOTTOM,
                _ => TableStyle::VA_BASELINE,
            };
        }
        "border-spacing" => {
            let first = v.split_whitespace().next().unwrap_or("");
            if let Some(px) = parse_px(first, c.font_size_px) {
                c.table.border_spacing = px.min(MAX_SPACING) as u16;
            }
        }
        "border-collapse" => c.table.border_collapse = v.eq_ignore_ascii_case("collapse"),
        _ => return false,
    }
    true
}
