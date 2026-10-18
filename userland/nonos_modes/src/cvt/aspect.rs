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

//! The vertical sync width CVT ties to the picture's aspect ratio (VESA
//! CVT 1.2 section 3.3, Table 3-2). It lets a monitor recognise a CVT
//! timing's aspect ratio from the sync alone.

/// The CVT character cell: widths and horizontal blanking come in 8s.
pub const CELL: u32 = 8;

/// Lines of vertical sync for a `width` (already a multiple of the cell)
/// by `height` picture: the ratio whose width, cut to the cell, matches.
pub fn v_sync(width: u32, height: u32) -> u32 {
    const RATIOS: [(u32, u32, u32); 5] =
        [(4, 3, 4), (16, 9, 5), (16, 10, 6), (5, 4, 7), (15, 9, 7)];
    for (num, den, lines) in RATIOS {
        if height as u64 * num as u64 / den as u64 / CELL as u64 * CELL as u64 == width as u64 {
            return lines;
        }
    }
    10
}
