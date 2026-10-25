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
//! Vol 12's combo PHY voltage swing tables (pp. 393 to 395), as printed.
//! Each row: Swing Sel (DW2, four bits), N Scalar (DW7), Cursor Coeff and
//! Post Cursor 1 (DW4); 2 Tap Disable (DW5) is 0 in every row used here.
//!
//! Vol 12 prints two different tables both headed "DP HBR2" (pp. 393 and
//! 394), and does not say which parts each is for. Neither is used: a
//! DisplayPort link above 2.7 Gbit/s on a combo port is refused until that
//! is settled. eDP's HBR3 rows need the board's VBT to allow HBR3, and the
//! optimised eDP row needs the VBT too (p. 395); neither is used yet.

/// One row of a swing table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Swing {
    pub sel: u32,
    pub n_scalar: u32,
    pub cursor: u32,
    pub post1: u32,
}

pub(super) const fn s(sel: u32, n_scalar: u32, cursor: u32, post1: u32) -> Swing {
    Swing { sel, n_scalar, cursor, post1 }
}

/// "DP up to HBR", by (voltage swing level, pre-emphasis level), in the
/// table's order: (0, 0) to (0, 3), (1, 0) to (1, 2), (2, 0), (2, 1), (3, 0).
pub const DP_HBR: [Swing; 10] = [
    s(0b1010, 0x32, 0x3F, 0x00),
    s(0b1010, 0x4F, 0x37, 0x08),
    s(0b1100, 0x71, 0x2F, 0x10),
    s(0b0110, 0x7D, 0x2B, 0x14),
    s(0b1010, 0x4C, 0x3F, 0x00),
    s(0b1100, 0x73, 0x34, 0x0B),
    s(0b0110, 0x7F, 0x2F, 0x10),
    s(0b1100, 0x6C, 0x3C, 0x03),
    s(0b0110, 0x7F, 0x35, 0x0A),
    s(0b0110, 0x7F, 0x3F, 0x00),
];

/// "eDP up to HBR2", in the same order.
pub const EDP_HBR2: [Swing; 10] = [
    s(0b0000, 0x7F, 0x3F, 0x00),
    s(0b1000, 0x7F, 0x38, 0x07),
    s(0b0001, 0x7F, 0x33, 0x0C),
    s(0b1001, 0x7F, 0x31, 0x0E),
    s(0b1000, 0x7F, 0x3F, 0x00),
    s(0b0001, 0x7F, 0x38, 0x07),
    s(0b1001, 0x7F, 0x35, 0x0A),
    s(0b0001, 0x7F, 0x3F, 0x00),
    s(0b1001, 0x7F, 0x38, 0x07),
    s(0b1001, 0x7F, 0x3F, 0x00),
];
