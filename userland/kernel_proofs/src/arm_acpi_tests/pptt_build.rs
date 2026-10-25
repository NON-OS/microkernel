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

//! PPTT structures, and a package of two cores sharing an L2.

use super::build::*;

pub fn node(flags: u32, parent: u32, id: u32, resources: &[u32]) -> Vec<u8> {
    let r: Vec<u8> = resources.iter().flat_map(|x| u32le(*x)).collect();
    let rest =
        [vec![0, 0], u32le(flags), u32le(parent), u32le(id), u32le(resources.len() as u32), r];
    structure(0, &rest.concat())
}

pub fn cache(next: u32, size: u32, id: u32) -> Vec<u8> {
    let rest = [
        vec![0, 0],
        u32le(0xFF),
        u32le(next),
        u32le(size),
        u32le(64),
        vec![4, 0xA],
        u16le(64),
        u32le(id),
    ];
    structure(1, &rest.concat())
}

/// At 36 the L2 (28 bytes), at 64 the package (20), at 84 core 0's L1,
/// at 112 core 0 (24), at 136 core 1's L1, at 164 core 1 (24).
pub fn table() -> Vec<u8> {
    let parts = [
        cache(0, 1 << 20, 2),
        node(1 | 2, 0, 0, &[]),
        cache(36, 64 << 10, 10),
        node(2 | 8, 64, 0, &[84]),
        cache(36, 64 << 10, 11),
        node(2 | 8, 64, 1, &[136]),
    ];
    sdt(b"PPTT", 3, &parts.concat())
}
