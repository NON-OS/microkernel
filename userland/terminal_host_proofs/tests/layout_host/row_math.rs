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

//! Row placement and the rail scroll extent.

use crate::rows;

/// The regression guard for the block-chrome drift bug: one painter stepped
/// rows by a constant 15 while another stepped by the measured line height,
/// so the two agreed only at font_scale 2 and separated everywhere else.
#[test]
fn row_top_advances_by_the_measured_line_height() {
    for lh in [13u32, 15, 17, 20, 23, 27] {
        for i in 0u32..15 {
            assert_eq!(rows::row_top(i, 50, lh), 50 + i * lh, "row {} at line height {}", i, lh);
        }
    }
}

/// The rail is one scrolling column, so a section stack taller than the rail
/// must scroll exactly its overflow, and a short stack must not scroll at all.
#[test]
fn the_rail_scroll_extent_is_the_overflow_only() {
    assert_eq!(rows::scroll_max(900, 600), 300);
    assert_eq!(rows::scroll_max(400, 600), 0);
    assert_eq!(rows::scroll_clamp(999, 900, 600), 300);
    assert_eq!(rows::scroll_clamp(120, 900, 600), 120);
    assert_eq!(rows::scroll_clamp(5, 400, 600), 0);
}
