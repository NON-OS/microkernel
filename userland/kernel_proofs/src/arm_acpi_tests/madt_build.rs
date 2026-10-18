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

//! MADTs with GIC structures as firmware lays them out.

use super::build::*;

/// A GICC of `len` bytes (76, 80 or 82).
pub fn gicc(len: usize, uid: u32, mpidr: u64, flags: u32) -> Vec<u8> {
    let mut r = [vec![0, 0], u32le(uid), u32le(uid), u32le(flags), u32le(0), u32le(23)].concat();
    r.extend([u64le(0), u64le(0x2C00_0000), u64le(0), u64le(0), u32le(25)].concat());
    r.extend([u64le(0x2D00_0000 + u64::from(uid) * 0x2_0000), u64le(mpidr)].concat());
    r.extend([vec![1, 0], u16le(21), u16le(22)].concat());
    r.truncate(len - 2);
    structure(0x0B, &r)
}

pub fn madt(structures: &[Vec<u8>]) -> Vec<u8> {
    sdt(b"APIC", 6, &[u32le(0), u32le(0), structures.concat()].concat())
}
