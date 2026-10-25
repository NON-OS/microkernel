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
//! DisplayPort link rates on the combo PHY, from Vol 12's table of DPLL
//! values (p. 178, 179). The table is the PRM's; nothing is computed.

use super::dividers::Dividers;

/// The link bit rates the table lists, in Mbit/s per lane.
pub const RATES: [u32; 8] = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];

/// The table's dividers for `mbps` on a reference of `ref_khz`, or None
/// for a rate or reference it does not list. A 38.4 MHz reference shares
/// the 19.2 MHz row. Rates above 5.4 Gbit/s are listed here, but Vol 12
/// says the board's VBT must allow them (p. 170); that gate is the
/// caller's.
pub fn dividers(mbps: u32, ref_khz: u32) -> Option<Dividers> {
    let row = RATES.iter().position(|r| *r == mbps)?;
    let at24 = match ref_khz {
        24_000 => true,
        19_200 | 38_400 => false,
        _ => return None,
    };
    // DCO integer, fraction at 24 MHz; the same at 19.2 MHz; P, K, Q.
    const TABLE: [(u32, u32, u32, u32, u32, u32, u32); 8] = [
        (0x151, 0x4000, 0x1A5, 0x7000, 5, 2, 1), // 1.62
        (0x168, 0x0000, 0x1C2, 0x0000, 2, 2, 2), // 2.16
        (0x151, 0x4000, 0x1A5, 0x7000, 3, 2, 1), // 2.7
        (0x151, 0x4000, 0x1A5, 0x7000, 5, 1, 1), // 3.24
        (0x168, 0x0000, 0x1C2, 0x0000, 2, 2, 1), // 4.32
        (0x151, 0x4000, 0x1A5, 0x7000, 3, 1, 1), // 5.4
        (0x195, 0x0000, 0x1FA, 0x2000, 3, 1, 1), // 6.48
        (0x151, 0x4000, 0x1A5, 0x7000, 2, 1, 1), // 8.1
    ];
    let (i24, f24, i19, f19, p, k, q) = TABLE[row];
    let (integer, fraction) = if at24 { (i24, f24) } else { (i19, f19) };
    Some(Dividers { integer, fraction, p, q, k })
}
