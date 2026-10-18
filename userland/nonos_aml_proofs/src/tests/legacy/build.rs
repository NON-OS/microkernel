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

//! The small and large resource descriptors the legacy proofs use.

use crate::enc::*;

pub fn io(base: u16, len: u8) -> Vec<u8> {
    let b = base.to_le_bytes();
    vec![0x47, 1, b[0], b[1], b[0], b[1], 1, len]
}

pub fn fixed_io(base: u16, len: u8) -> Vec<u8> {
    let b = base.to_le_bytes();
    vec![0x4B, b[0], b[1], len]
}

pub fn irq(n: u8) -> Vec<u8> {
    let m = (1u16 << n).to_le_bytes();
    vec![0x22, m[0], m[1]]
}

pub fn mem32(base: u32, len: u32) -> Vec<u8> {
    [vec![0x86, 9, 0, 1], base.to_le_bytes().to_vec(), len.to_le_bytes().to_vec()].concat()
}

pub fn crs(parts: &[Vec<u8>]) -> Vec<u8> {
    let t = [parts.concat(), vec![0x79, 0]].concat();
    name_obj("_CRS", &buffer(t.len() as u64, &t))
}
