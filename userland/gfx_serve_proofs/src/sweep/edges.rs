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

//! The edges of u32 a size can sit on, for the sweeps that hold every
//! product of sizes (G-70): zero and one, the small ones a mode has, the
//! powers a product turns over at (2^16, 2^24, 2^28, 2^30, 2^31) with
//! their neighbours, and the top.

pub const EDGES: [u32; 28] = [
    0,
    1,
    2,
    3,
    4,
    63,
    64,
    65,
    640,
    4095,
    4096,
    8192,
    65535,
    65536,
    1 << 24,
    1 << 28,
    (1 << 30) - 1,
    1 << 30,
    (1 << 30) + 1,
    (1 << 31) - 1,
    1 << 31,
    (1 << 31) + 1,
    u32::MAX / 4,
    u32::MAX / 4 + 1,
    u32::MAX / 2,
    u32::MAX - 63,
    u32::MAX - 1,
    u32::MAX,
];

/// Every ordered pair of edges.
pub fn pairs() -> impl Iterator<Item = (u32, u32)> {
    EDGES.iter().flat_map(|&a| EDGES.iter().map(move |&b| (a, b)))
}
