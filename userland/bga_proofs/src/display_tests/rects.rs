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

//! Rectangles of a `W` x `H` mode as (x, y, width, height): at and one past
//! every edge, empty, and with corners or sides at u32::MAX or summing past
//! it, or whose last line's start overflows a 32-bit count of pixels.

pub const W: u32 = 64;
pub const H: u32 = 48;
const M: u32 = u32::MAX;

pub const INSIDE: [(u32, u32, u32, u32); 7] = [
    (0, 0, W, H),
    (0, 0, 1, 1),
    (W - 1, 0, 1, 1),
    (0, H - 1, 1, 1),
    (W - 1, H - 1, 1, 1),
    (0, H - 1, W, 1),
    (W - 1, 0, 1, H),
];

pub const OUTSIDE: [(u32, u32, u32, u32); 20] = [
    (W, 0, 1, 1),
    (0, H, 1, 1),
    (W - 1, 0, 2, 1),
    (0, H - 1, 1, 2),
    (0, 0, W + 1, 1),
    (0, 0, 1, H + 1),
    (W, H, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 1, 0),
    (0, 0, 0, 0),
    (M, 0, 1, 1),
    (0, M, 1, 1),
    (0, 0, M, 1),
    (0, 0, 1, M),
    (1, 0, M, 1),
    (0, 1, 1, M),
    (M, M, M, M),
    (M, M, 1, 1),
    (0, M / W, 1, 2),
    (W - 1, M / W, 1, W),
];
