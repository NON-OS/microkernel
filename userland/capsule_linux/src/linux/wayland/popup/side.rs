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

//! The sides an anchor or a gravity names, and whether a positioner can
//! place anything. Pure, so the host proofs hold it.

use super::positioner::Positioner;

/// An anchor or gravity as the side it names on each axis: -1 left or top,
/// 1 right or bottom, 0 the middle.
pub fn sides(edge: u32) -> (i8, i8) {
    match edge {
        1 => (0, -1),
        2 => (0, 1),
        3 => (-1, 0),
        4 => (1, 0),
        5 => (-1, -1),
        6 => (-1, 1),
        7 => (1, -1),
        8 => (1, 1),
        _ => (0, 0),
    }
}

/// Whether the positioner can place anything: a size was set.
pub fn complete(p: &Positioner) -> bool {
    p.size.0 > 0 && p.size.1 > 0
}
