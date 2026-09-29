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

mod table_props;

use crate::browser::css::computed::Computed;

/* Box roles a display value sets: block-level, flex, grid, inline-block,
 * table, table row, table cell. */
type Role = (bool, bool, bool, bool, bool, bool, bool);

/* The display property routes a box to its formatting context. A later
 * display value replaces an earlier one whole, none included, so
 * display:none in one rule and display:block in a more specific one
 * shows the box. An unknown value is invalid and changes nothing. */
pub(super) fn apply_display(c: &mut Computed, name: &str, value: &str) -> bool {
    if name != "display" {
        return table_props::apply_table(c, name, value);
    }
    let v = value.trim().to_ascii_lowercase();
    let role: Role = match v.as_str() {
        "none" | "table-column" | "table-column-group" => {
            c.display_none = true;
            return true;
        }
        "contents" => {
            (c.display_none, c.is_contents) = (false, true);
            return true;
        }
        "block" | "list-item" | "flow-root" | "table-caption" | "-webkit-box" | "run-in" => {
            (true, false, false, false, false, false, false)
        }
        "flex" | "-webkit-flex" => (true, true, false, false, false, false, false),
        "inline-flex" | "-webkit-inline-flex" => (false, true, false, false, false, false, false),
        "grid" => (true, false, true, false, false, false, false),
        "inline-grid" => (false, false, true, false, false, false, false),
        "inline" | "ruby" | "inline-list-item" => (false, false, false, false, false, false, false),
        "inline-block" => (false, false, false, true, false, false, false),
        "table" | "inline-table" => (true, false, false, false, true, false, false),
        "table-row" => (true, false, false, false, false, true, false),
        "table-row-group" | "table-header-group" | "table-footer-group" => {
            (true, false, false, false, false, false, false)
        }
        "table-cell" => (true, false, false, false, false, false, true),
        _ => return true,
    };
    (c.display_none, c.is_contents) = (false, false);
    (c.is_block, c.is_flex, c.is_grid, c.is_inline_block) = (role.0, role.1, role.2, role.3);
    (c.is_table, c.is_table_row, c.is_table_cell) = (role.4, role.5, role.6);
    c.table.caption = v == "table-caption";
    true
}
