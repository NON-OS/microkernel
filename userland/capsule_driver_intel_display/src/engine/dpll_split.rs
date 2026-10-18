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

//! Splitting the HDMI divider into the DPLL's three stages (SKL PRM Vol
//! 12, DPLL HDMI mode): P0 is 1, 2, 3 or 7; P1 any count to 255; P2 is 1,
//! 2, 3 or 5.

/// (P0, P1, P2) for a divider `dpll_hdmi` can return.
pub fn split(p: u32) -> Option<(u32, u32, u32)> {
    if p.is_multiple_of(2) {
        let half = p / 2;
        return match half {
            1 | 2 | 3 | 5 => Some((2, 1, half)),
            h if h.is_multiple_of(2) => Some((2, h / 2, 2)),
            h if h.is_multiple_of(3) => Some((3, h / 3, 2)),
            h if h.is_multiple_of(7) => Some((7, h / 7, 2)),
            _ => None,
        };
    }
    match p {
        3 | 9 => Some((3, 1, p / 3)),
        // P0 has no 5: five is taken by P2.
        5 => Some((1, 1, 5)),
        7 => Some((7, 1, 1)),
        15 => Some((3, 1, 5)),
        21 => Some((7, 1, 3)),
        35 => Some((7, 1, 5)),
        _ => None,
    }
}
