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
//! Vol 12's HDMI rows of the combo PHY swing table (p. 395), and where a
//! DisplayPort level pair sits in the DP and eDP tables.

use super::swing_table::{s, Swing};

/// The HDMI rows, in the table's order (p. 395); the fourth is the one
/// Vol 12 names for ALS and retimers.
pub const HDMI: [Swing; 7] = [
    s(0b1010, 0x60, 0x3F, 0x00),
    s(0b1011, 0x73, 0x36, 0x09),
    s(0b0110, 0x7F, 0x31, 0x0E),
    s(0b1011, 0x73, 0x3F, 0x00),
    s(0b0110, 0x7F, 0x37, 0x08),
    s(0b0110, 0x7F, 0x3F, 0x00),
    s(0b0110, 0x7F, 0x35, 0x0A),
];

/// The row for DisplayPort levels `vs` and `pe`: their sum is at most 3
/// (p. 393), and the rows run (0, 0..3), (1, 0..2), (2, 0..1), (3, 0).
pub fn dp_row(vs: u32, pe: u32) -> Option<usize> {
    let start = [0, 4, 7, 9];
    (vs <= 3 && pe <= 3 - vs).then(|| start[vs as usize] + pe as usize)
}
