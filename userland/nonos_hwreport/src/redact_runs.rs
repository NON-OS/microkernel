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

//! The masked values that are runs rather than fixed shapes.

fn hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

/// A run of hex, colons and dots with seven colons or a "::" (IPv6), or a
/// run of twelve or more hex digits (a key, digest, serial or address).
pub(crate) fn run_at(line: &[u8], i: usize) -> usize {
    let v6 = line[i..].iter().take_while(|b| hex(**b) || **b == b':' || **b == b'.').count();
    let w = &line[i..i + v6];
    let colons = w.iter().filter(|b| **b == b':').count();
    if colons >= 7 || (colons >= 2 && w.windows(2).any(|p| p == b"::")) {
        return v6;
    }
    let digits = line[i..].iter().take_while(|b| hex(**b)).count();
    if digits >= 12 {
        digits
    } else {
        0
    }
}
