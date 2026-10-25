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
//! A listed HDMI divider as the P, Q and K that make it, by the rules Vol 12
//! gives (p. 181, 182): 2 alone, 3, 5 and 7 as P, 9, 15 and 21 as P with K
//! 3, and every other even divider as Q times 4, 6, 10 or 14 with K 2, the
//! first of those that divides it.

/// P, Q and K for a listed divider (p. 181, 182).
pub fn split(div: u32) -> (u32, u32, u32) {
    match div {
        2 => (2, 1, 1),
        3 | 5 | 7 => (div, 1, 1),
        9 | 15 | 21 => (div / 3, 1, 3),
        d if d % 4 == 0 => (2, d / 4, 2),
        d if d % 6 == 0 => (3, d / 6, 2),
        d if d % 10 == 0 => (5, d / 10, 2),
        d => (7, d / 14, 2),
    }
}
